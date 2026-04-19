use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use sqlx::{PgPool};
use thiserror::Error;
use std::fmt::Display;
use time::PrimitiveDateTime;
use typesafe_builder::*;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    price_cents: i32,
    created_at: PrimitiveDateTime,
    updated_at: PrimitiveDateTime,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Product: {} - {}\nPrice: {}\n",
            self.name, self.id, self.price_cents
        )
    }
}

#[derive(Builder, Debug, Serialize, Deserialize)]
pub struct NewProduct {
    #[builder(required, into)]
    name: String,
    #[builder(required)]
    price_cents: i32,
    #[builder(optional, into)]
    description: Option<String>,
}

impl NewProduct {
    pub fn validate(self) -> Result<Self, AppError> {
        if self.price_cents < 0 {
            return Err(ValidationError::InvalidPrice(self.price_cents).into());
        }
        if self.name.is_empty() {
            return Err(ValidationError::EmptyName.into());
        }
        Ok(self)
    }
}

pub async fn create_product(
    pool: &PgPool,
    new_product: NewProduct
) -> anyhow::Result<Product, AppError> {
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
    ).fetch_one(pool).await?;

    Ok(product)
}

pub async fn find_all(pool: &PgPool) -> anyhow::Result<Vec<Product>, AppError> {
    let products = sqlx::query_as!(
        Product,
        r#"
            SELECT * FROM products
        "#
    ).fetch_all(pool).await?;

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
    ).fetch_all(pool).await?;

    Ok(products)
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid price {0}")]
    InvalidPrice(i32),
    #[error("Name cannot be empty")]
    EmptyName,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Validation(#[from] ValidationError)
}
