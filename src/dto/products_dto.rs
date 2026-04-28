use crate::{AppError, ValidationError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn new(value: String) -> Result<Self, AppError> {
        if value.is_empty() {
            return Err(ValidationError::EmptyName.into());
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
pub struct PriceCents(i32);

impl PriceCents {
    fn new(value: i32) -> Result<Self, AppError> {
        if value < 0 {
            return Err(ValidationError::InvalidPrice(value).into());
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProduct {
    pub(crate) name: NonEmptyString,
    pub(crate) price_cents: PriceCents,
    pub(crate) description: Option<String>,
}

impl NewProduct {
    pub fn new(
        name: impl Into<String>,
        price_cents: i32,
        description: Option<String>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            name: NonEmptyString::new(name.into())?,
            price_cents: PriceCents::new(price_cents)?,
            description,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProduct {
    pub(crate) name: Option<NonEmptyString>,
    pub(crate) price_cents: Option<PriceCents>,
    pub(crate) description: Option<String>,
}

impl UpdateProduct {
    pub fn new(
        name: Option<String>,
        price_cents: Option<i32>,
        description: Option<String>,
    ) -> Result<Self, AppError> {
        Ok(Self {
            name: name.map(NonEmptyString::new).transpose()?,
            price_cents: price_cents.map(PriceCents::new).transpose()?,
            description,
        })
    }
}
