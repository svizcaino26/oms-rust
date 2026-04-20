pub mod repository;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use thiserror::Error;
use time::PrimitiveDateTime;
// use typesafe_builder::*;

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

// #[derive(Builder, Debug, Serialize, Deserialize)]
#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    // #[builder(required, into)]
    name: String,
    // #[builder(required)]
    price_cents: i32,
    // #[builder(optional, into)]
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

    pub fn new(
        name: impl Into<String>,
        price_cents: i32,
        description: Option<String>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            name: name.into(),
            price_cents,
            description,
        }
        .validate()?)
    }
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
    Validation(#[from] ValidationError),
}
