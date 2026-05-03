use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use time::PrimitiveDateTime;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Product {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) price_cents: i32,
    pub(crate) created_at: PrimitiveDateTime,
    pub(crate) updated_at: PrimitiveDateTime,
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

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Inventory {
    product_id: i32,
    quantity: i32,
    reserved: i32,
    updated_at: PrimitiveDateTime,
}

impl Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nAvailable: {}\nReserved:{}",
            self.product_id, self.quantity, self.reserved
        )
    }
}
