use crate::config::{LogsConfig, LogsLevel};
use anyhow::Context;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tracing_appender::{non_blocking, non_blocking::WorkerGuard};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn init_logs(config: &LogsConfig) -> anyhow::Result<Option<WorkerGuard>> {
    if !config.enabled {
        return Ok(None);
    }

    let env_filter = tracing_subscriber::EnvFilter::new(match config.level {
        LogsLevel::Error => "error",
        LogsLevel::Warn => "warn",
        LogsLevel::Info => "info",
        LogsLevel::Debug => "debug",
        LogsLevel::Trace => "trace",
    });

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
        let exe_path =
            std::env::current_exe().context("Unable to get the path of the executable file")?;
        let exe_folder = exe_path
            .parent()
            .context("Unable to get the directory of the executable file")?;
        let logs_folder = exe_folder.join(&config.file.path);

        if fs::metadata(&logs_folder).await.is_err() {
            fs::create_dir_all(&logs_folder).await.with_context(|| {
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
