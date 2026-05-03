use serde::{Deserialize, Serialize};

use crate::errors::{AppError, ValidationError};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn new(value: String) -> Result<Self, AppError> {
        if value.is_empty() {
            return Err(ValidationError::EmptyName.into());
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceCents(i32);

impl PriceCents {
    pub(crate) fn new(value: i32) -> Result<Self, AppError> {
        if value < 0 {
            return Err(ValidationError::InvalidPrice(value).into());
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct InventoryQuantity(i32);

impl InventoryQuantity {
    pub fn new(value: i32) -> Result<Self, AppError> {
        if value < 0 {
            return Err(ValidationError::NegativeInventoryQuantity(value).into());
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}
