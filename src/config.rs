use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub config_dir: PathBuf,
    pub box_pool_size: u32,
    pub api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let port = std::env::var("ISOLATE_SANDBOX_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);

        let config_dir = std::env::var("ISOLATE_SANDBOX_CONFIG_DIR")
            .ok()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("./config"));

        let box_pool_size = std::env::var("ISOLATE_SANDBOX_BOX_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        let api_key = std::env::var("ISOLATE_SANDBOX_API_KEY").ok();

        Self {
            port,
            config_dir,
            box_pool_size,
            api_key,
        }
    }
}

