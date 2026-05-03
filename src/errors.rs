use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid price {0}")]
    InvalidPrice(i32),
    #[error("Name cannot be empty")]
    EmptyName,
    #[error("Inventory quantity cannot be negative - got: {0}")]
    NegativeInventoryQuantity(i32),
    #[error("Update request contains no fields")]
    EmptyQuery,
}

#[derive(Debug, Error)]
pub enum NotFoundError {
    #[error("Product not found - id: {0}")]
    ProductNotFound(i32),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    NotFound(#[from] NotFoundError),
    #[error("{0}")]
    Validation(#[from] ValidationError),
}
