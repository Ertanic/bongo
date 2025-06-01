use anyhow::Context;
use crate::logs::init_logs;

mod logs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_logs().await.context("Failed to init logs system")?;

    tracing::info!("Application is starting...");
    tracing::info!("Application is running");

    Ok(())
}
