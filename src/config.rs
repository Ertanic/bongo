use anyhow::{Context, bail};
use knus::{Decode, DecodeScalar};
use std::path::Path;
use tokio::fs;

pub async fn load_config(path: &Path) -> anyhow::Result<Config> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path
        .parent()
        .expect("Unable to get the directory of the executable file");
    let config_path = exe_dir.join(path);

    let content = fs::read_to_string(&config_path)
        .await
        .with_context(|| format!("Unable to load config file: {}", config_path.display()))?;

    match knus::parse(
        config_path
            .to_str()
            .expect("Unable to convert a path to a string"),
        &content,
    ) {
        Ok(config) => Ok(config),
        Err(err) => {
            let error = miette::Report::new(err);
            eprintln!("{error:?}");
            bail!("Failed to parse the config file: {}", config_path.display())
        }
    }
}

#[derive(Decode, Default)]
pub struct Config {
    #[knus(child, default)]
    pub logs: LogsConfig,

    #[knus(child, default)]
    pub app: AppConfig,
}

#[derive(Decode, Default)]
pub struct LogsConfig {
    #[knus(property, default = true)]
    pub enabled: bool,

    #[knus(child, unwrap(argument))]
    pub level: LogsLevel,

    #[knus(child, default)]
    pub file: FileLogsConfig,
}

#[derive(DecodeScalar, Default)]
pub enum LogsLevel {
    Error,
    Warn,
    #[default]
    Info,
    Debug,
    Trace,
}

#[derive(Decode, Default)]
pub struct FileLogsConfig {
    #[knus(property, default = true)]
    pub enabled: bool,

    #[knus(property, default="logs".into())]
    pub path: String,
}

#[derive(Decode, Default)]
pub struct AppConfig {
    #[knus(child, unwrap(argument), default = 80)]
    pub port: u16,

    #[knus(child, unwrap(children))]
    pub routes: Vec<RouteConfig>,
}

#[derive(Decode)]
pub enum RouteConfig {
    File(FileRouteConfig),
    Dir(#[knus(argument)] String, #[knus(argument)] String),
    FallbackFile(#[knus(argument)] String),
}

#[derive(Decode)]
pub struct FileRouteConfig {
    #[knus(argument)]
    pub path: String,

    #[knus(argument)]
    pub file: String,

    #[knus(children)]
    pub nest_routes: Vec<RouteConfig>,
}