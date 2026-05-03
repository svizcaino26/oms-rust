use sqlx::PgPool;

use crate::{errors::AppError, repository::product_repository::find_by_id, types::InventoryQuantity};

pub async fn increase_stock(pool: &PgPool, id: i32, quantity: InventoryQuantity) -> Result<bool, AppError> {
    find_by_id(pool, id).await?;

    let result = sqlx::query!(
        r#"
            INSERT INTO inventory (product_id, quantity)
            VALUES ($1, $2)
            ON CONFLICT (product_id) DO UPDATE SET quantity = inventory.quantity + $2
        "#,
        id, quantity.value()
    ).execute(pool).await?;

    Ok(result.rows_affected() > 0)
}

pub async fn reserve_stock() {}
pub async fn decrease_stock() {}
pub async fn stock_by_id() {}
pub async fn all_stock() {}
