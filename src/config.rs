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

        config_path
            .to_str()
            .expect("Unable to convert a path to a string"),
        &content,
}

#[derive(Decode, Default)]
pub struct Config {
    #[knus(child, default)]
    pub logs: LogsConfig,

    #[knus(child, default)]
    pub app: AppConfig,
}

pub struct LogsConfig {
    #[knus(property, default = true)]
    pub enabled: bool,

    #[knus(child, unwrap(argument))]
    pub level: LogsLevel,

    #[knus(child, default)]
    pub file: FileLogsConfig,
}

}

pub struct FileLogsConfig {
    #[knus(property, default = true)]
    pub enabled: bool,

    #[knus(property, default="logs".into())]
    pub path: String,
}

        }
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