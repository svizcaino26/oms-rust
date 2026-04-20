use crate::{AppError, NewProduct, Product};
use sqlx::PgPool;

pub async fn save(pool: &PgPool, new_product: NewProduct) -> anyhow::Result<Product, AppError> {
    let new_product = new_product.validate()?;
    let product = sqlx::query_as!(
        Product,
        r#"
            INSERT INTO products (name, price_cents, description)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        new_product.name,
        new_product.price_cents,
        new_product.description
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn find_all(pool: &PgPool) -> anyhow::Result<Vec<Product>, AppError> {
    let products = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn find_all_limited(pool: &PgPool, limit: i64) -> anyhow::Result<Vec<Product>, AppError> {
    let products = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products
            LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(products)
}

pub async fn find_by_id(pool: &PgPool, id: i32) -> anyhow::Result<Option<Product>, AppError> {
    let product = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(product)
}

pub async fn delete(pool: &PgPool, product: Product) -> anyhow::Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM products
            WHERE id = $1
        "#,
        product.id
    )
    .execute(pool)
    .await?;
    Ok(())
}
