use crate::{AppError, Product, ValidationError};
use crate::dto::products_dto::{NewProduct, UpdateProduct};
use sqlx::{PgPool, Postgres, QueryBuilder};

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

pub async fn update_product(pool: &PgPool, id: i32, update_product: UpdateProduct) -> Result<bool, AppError> {
    let mut field_inserted = false;
    let mut qb: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE products SET ");

    if let Some(name) = update_product.name {
        let name = String::from(name.as_str());
        qb.push("name = ").push_bind(name);
        field_inserted = true;
    }

    if let Some(price_cents) = update_product.price_cents {
        if field_inserted {
            qb.push(", ");
        }
        qb.push("price_cents = ").push_bind(price_cents.value());
        field_inserted = true;
    }

    if let Some(description) = update_product.description {
        if field_inserted {
            qb.push(", ");
        }
        qb.push("description = ").push_bind(description);
        field_inserted = true;
    }

    qb.push(" WHERE id = ").push_bind(id);

    if !field_inserted {
        return Err(ValidationError::EmptyQuery.into());
    }
    let result = qb.build().execute(pool).await?;
    
    Ok(result.rows_affected() > 0)
}
