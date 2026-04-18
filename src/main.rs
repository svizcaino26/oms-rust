use oms_rust::{NewProductBuilder, create_product};
use std::env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();

    let db = PgPool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let new_product = NewProductBuilder::new()
        .with_name("big monitor")
        .with_price_cents(9900)
        .with_description("Big monitor")
        .build();

    let product = create_product(&db, new_product).await?;

    println!("{product:?}");
    Ok(())
}
