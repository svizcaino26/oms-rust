use crate::{AppError, Product};
use crate::dto::products_dto::NewProduct;
use sqlx::PgPool;

pub async fn save(pool: &PgPool, new_product: NewProduct) -> Result<Product, AppError> {
    let product = sqlx::query_as!(
        Product,
        r#"
            INSERT INTO products (name, price_cents, description)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        new_product.name.as_str(),
        new_product.price_cents.value(),
        new_product.description
    )
    .fetch_one(pool)
    .await?;

    Ok(product)
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<Product>, AppError> {
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

pub async fn find_all_limited(pool: &PgPool, limit: i64) -> Result<Vec<Product>, AppError> {
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

pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Product>, AppError> {
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

pub async fn delete(pool: &PgPool, id: i32) -> Result<bool, AppError> {
    let result = sqlx::query!(
        r#"
            DELETE FROM products
            WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn update_product(pool: &PgPool, id: i32, price_cents: i32) -> Result<bool, AppError> {
    let result = sqlx::query!(
        r#"
            UPDATE products
            SET price_cents = $1
            WHERE id = $2
        "#,
        price_cents, id
    ).execute(pool).await?;

    Ok(result.rows_affected() > 0)
}

