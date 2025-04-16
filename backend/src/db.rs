use std::env;

use sqlx::postgres::PgPoolOptions;

const DATABASE_URL_ENV: &str = "DATABASE_URL";
const MAX_CONNECTIONS: u32 = 4;

pub type Pool = sqlx::Pool<sqlx::Postgres>;

pub async fn create_pool() -> Option<Pool> {
    let db_url = env::var(DATABASE_URL_ENV).ok()?;

    PgPoolOptions::new()
        .max_connections(MAX_CONNECTIONS)
        .connect(db_url.as_str())
        .await
        .ok()
}
