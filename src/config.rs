use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub telegram_bot_token: String,
    pub telegram_announcements_channel_id: i64,
    pub telegram_logs_channel_id: i64,
    pub azure_face_api_key: String,
    pub azure_face_endpoint: String,
    pub rust_env: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").or_else(|_| -> Result<String> {
            let user = env::var("DB_USER")?;
            let pass = env::var("DB_PASSWORD")?;
            let host = env::var("DB_HOST")?;
            let port = env::var("DB_PORT")?;
            let name = env::var("DB_NAME")?;
            Ok(format!(
                "postgres://{}:{}@{}:{}/{}",
                user, pass, host, port, name
            ))
        })?;

        Ok(Config {
            database_url,
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: env::var("DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .context("DB_PORT must be a valid u16")?,
            db_name: env::var("DB_NAME").unwrap_or_else(|_| "telegram_bot".to_string()),
            db_user: env::var("DB_USER").unwrap_or_else(|_| "botuser".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or_else(|_| "botpass".to_string()),
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")
                .context("TELEGRAM_BOT_TOKEN environment variable not set")?,
            telegram_announcements_channel_id: env::var("TELEGRAM_ANNOUNCEMENTS_CHANNEL_ID")
                .context("TELEGRAM_ANNOUNCEMENTS_CHANNEL_ID not set")?
                .parse()
                .context("TELEGRAM_ANNOUNCEMENTS_CHANNEL_ID must be a valid i64")?,
            telegram_logs_channel_id: env::var("TELEGRAM_LOGS_CHANNEL_ID")
                .context("TELEGRAM_LOGS_CHANNEL_ID not set")?
                .parse()
                .context("TELEGRAM_LOGS_CHANNEL_ID must be a valid i64")?,
            azure_face_api_key: env::var("AZURE_FACE_API_KEY")
                .context("AZURE_FACE_API_KEY not set")?,
            azure_face_endpoint: env::var("AZURE_FACE_ENDPOINT")
                .context("AZURE_FACE_ENDPOINT not set")?,
            rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
        })
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == "production"
    }
}
