mod bot;
mod config;
mod domain;
mod error;
mod infrastructure;
mod utils;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;
use teloxide::types::Update;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    db: Pool<Postgres>,
    config: Arc<config::Config>,
}

#[tokio::main]
async fn main() -> Result<()> {
    utils::logger::init();

    info!("🤖 Starting Telegram Bot with Webhook...");

    let config = config::Config::from_env()?;
    info!("✓ Configuration loaded");

    let db = infrastructure::database::init(&config).await?;
    info!("✓ Database connected");

    let state = AppState {
        db,
        config: Arc::new(config.clone()),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/telegram/update", post(handle_telegram_update))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    info!("✓ Server listening on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok" }))
}

async fn handle_telegram_update(
    State(_state): State<AppState>,
    Json(_update): Json<Update>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({})))
}
