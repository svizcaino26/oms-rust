// use oms_rust::NewProductBuilder;
use oms_rust::NewProduct;
use sqlx::PgPool;
use std::env;

use oms_rust::repository::product_repository::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().unwrap();

    let db = PgPool::connect_lazy(
        &env::var("DATABASE_URL").expect("failed to load DATABASE_URL from env"),
    )
    .expect("Failed to initialize DB pool");

    let new_product = NewProduct::new("PS$ controller", 7400, None)?;

    let product = save(&db, new_product).await?;
    println!("{product}");

    let all_products = find_all(&db).await?;
    println!("List of all products");
    for p in all_products {
        println!("{p}");
    }

    // // let id = 2;

    // // if let Some(product) = find_by_id(&db, id).await? {
    // //     println!("{product}");
    // //     delete(&db, product).await?;
    // // }

    // let all_products = find_all(&db).await?;
    // println!("List of all products after delete");
    // for p in all_products {
    //     println!("{p}");
    // }

    Ok(())
}
