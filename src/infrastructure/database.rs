use crate::config::Config;
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init(config: &Config) -> anyhow::Result<Pool<Postgres>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .context("Failed to connect to PostgreSQL database")?;

    sqlx::query("SELECT 1")
        .execute(&pool)
        .await
        .context("Failed to verify database connection")?;

    Ok(pool)
}
