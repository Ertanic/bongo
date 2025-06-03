#[cfg(not(debug_assertions))]
use anyhow::Context;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let env_filter = tracing_subscriber::EnvFilter::new("trace");

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_level(true)
        .with_target(true);

    let register = tracing_subscriber::registry().with(console_layer);

    if !config.file.enabled {
        register.with(env_filter).init();
        return Ok(None);
    }

    if let Ok(time) = SystemTime::now().duration_since(UNIX_EPOCH) {
        if fs::metadata(LOGS_FOLDER).await.is_err() {
            fs::create_dir(LOGS_FOLDER).await.with_context(|| {
                format!(
                    "It is not possible to create a logs folder in the path: {}",
                    logs_folder.display()
                )
            })?;
        }

        let log_file_path = logs_folder.join(format!("{}.log", time.as_secs()));
        let log_file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file_path)
            .with_context(|| format!("Failed to create log file: {}", log_file_path.display()))?;

        let (non_blocking, _guard) = non_blocking(log_file);
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)
            .with_level(true)
            .with_target(true);

        register.with(file_layer).with(env_filter).init();

        Ok(Some(_guard))
    } else {
        register.with(env_filter).init();

        Ok(None)
    }
}
