use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Telegram error: {0}")]
    TelegramError(String),

    #[error("Azure Face API error: {0}")]
    AzureError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
