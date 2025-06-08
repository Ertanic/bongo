use crate::config::load_config;
use crate::logs::init_logs;
use crate::routes::register_routes;
use anyhow::Context;
use axum::Router;
use std::net::Ipv4Addr;
use tower_http::trace::TraceLayer;

mod config;
mod logs;
mod routes;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config("config.kdl".as_ref()).await?;
    let _guard = init_logs(&config.logs)
        .await
        .context("Failed to init logs system")?;

    tracing::info!("Application is starting...");

    let app = register_routes(Router::new(), config.app.routes)
        .context("Unable to register routes")?
        .layer(TraceLayer::new_for_http());

    tracing::info!("Routes have been registered");

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), config.app.port))
        .await
        .with_context(|| format!("Failed to bind {} port", config.app.port))?;

    tracing::info!("Port {} has been successfully bind", config.app.port);
    tracing::info!("Application is running");

    axum::serve(listener, app).await?;

    Ok(())
}
