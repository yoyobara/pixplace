mod models;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    println!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Database connection established.");

    println!("Running migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    println!("Migrations completed successfully.");

    println!("Hello, world!");

    Ok(())
}
