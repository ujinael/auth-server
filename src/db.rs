use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub type DbPool = PgPool;

pub async fn init_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .expect("Failed to create pool.")
}
