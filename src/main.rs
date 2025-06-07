use crate::config::load_config;
use crate::logs::init_logs;
use anyhow::Context;
use axum::Router;
use std::net::Ipv4Addr;
use tracing::Level;

mod config;
mod logs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config("config.kdl".as_ref()).await?;
    let _guard = init_logs(&config.logs)
        .await
        .context("Failed to init logs system")?;

    tracing::info!("Application is starting...");

    let span = tracing::span!(Level::INFO, "app");
    let _enter = span.enter();

    let app = Router::new();

    tracing::info!("Routes have been registered");

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), config.app.port))
        .await
        .with_context(|| format!("Failed to bind {} port", config.app.port))?;

    tracing::info!("Port {} has been successfully bind", config.app.port);
    tracing::info!("Application is running");

    axum::serve(listener, app).await?;

    Ok(())
}
