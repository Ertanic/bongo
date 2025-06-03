use crate::config::load_config;
use crate::logs::init_logs;
use anyhow::Context;

mod config;
mod logs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = load_config("config.kdl".as_ref()).await?;
    let _guard = init_logs(&config.logs)
        .await
        .context("Failed to init logs system")?;

    tracing::info!("Application is starting...");
    tracing::info!("Application is running");

    Ok(())
}
