use anyhow::Context;
use tokio::fs;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;

const LOGS_FOLDER: &str = "logs";

async fn init_logs() -> anyhow::Result<Option<WorkerGuard>> {
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = tracing_subscriber::EnvFilter::new("trace");

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_level(true)
        .with_target(true);

    let register = tracing_subscriber::registry().with(console_layer);

    if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if fs::metadata(LOGS_FOLDER).await.is_err() {
            fs::create_dir(LOGS_FOLDER).await.with_context(|| {
                format!(
                    "It is not possible to create a logs folder in the path: {}",
                    LOGS_FOLDER
                )
            })?;
        }

        let log_file_path = format!("{}\\{}.log", LOGS_FOLDER, time.as_secs());
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)
            .with_context(|| format!("Failed to create log file: {}", log_file_path))?;

        let (non_blocking, _guard) = non_blocking(log_file);
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_level(true)
            .with_target(true);

        register
            .with(file_layer)
            .with(env_filter)
            .init();

        Ok(Some(_guard))
    } else {
        register
            .with(env_filter)
            .init();

        Ok(None)
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_logs().await.context("Failed to init logs system")?;

    tracing::info!("Application is starting...");
    tracing::info!("Application is running");

    Ok(())
}
