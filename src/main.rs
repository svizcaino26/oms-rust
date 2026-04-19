use oms_rust::{NewProductBuilder, create_product, find_all};
use std::env;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();

    let db = PgPool::connect_lazy(&env::var("DATABASE_URL")
        .expect("failed to load DATABASE_URL from env"))
        .expect("Failed to initialize DB pool");

    // let new_product = NewProductBuilder::new()
    //     .with_name("Oled TV")
    //     .with_price_cents(20000)
    //     .with_description("24 inch TV")
    //     .build();

    // let product = create_product(&db, new_product).await?;

    // println!("{product}");

    let all_products = find_all(&db).await?;
    for p in all_products {
        println!("{p}");
    }
    Ok(())
}
