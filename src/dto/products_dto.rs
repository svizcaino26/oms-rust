use crate::{
    errors::AppError,
    types::{NonEmptyString, PriceCents},
};
use serde::{Deserialize, Serialize};

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
