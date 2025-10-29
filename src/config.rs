use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SandboxConfig {
    pub default_cg_mem: u32,      // Memory limit in KB (0 = unlimited)
    pub default_mem: u32,         // Memory limit in KB (0 = unlimited)
    pub default_time: u32,        // Time limit in seconds (0 = unlimited)
    pub default_wall_time: u32,   // Wall time limit in seconds (0 = unlimited)
    pub default_extra_time: u32,  // Extra time in seconds (0 = unlimited)
    pub default_stack: u32,       // Stack limit in KB (0 = unlimited)
    pub default_fsize: u32,       // File size limit in KB (0 = unlimited)
    pub default_open_files: u32,  // Open files limit (0 = unlimited)
    pub default_processes: u32,   // Processes limit (0 = unlimited, uses --processes without value)
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            default_cg_mem: 524288,      // 512MB
            default_mem: 512000,         // 512MB
            default_time: 30,            // 30 seconds
            default_wall_time: 60,       // 60 seconds
            default_extra_time: 10,      // 10 seconds
            default_stack: 128000,       // 128KB
            default_fsize: 102400,       // 100MB
            default_open_files: 64,      // 64 files
            default_processes: 0,        // Unlimited processes
        }
    }
}

impl SandboxConfig {
    pub fn from_env() -> Self {
        let default = Self::default();

        Self {
            default_cg_mem: std::env::var("ISOLATE_SANDBOX_DEFAULT_CG_MEM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_cg_mem),
            default_mem: std::env::var("ISOLATE_SANDBOX_DEFAULT_MEM")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_mem),
            default_time: std::env::var("ISOLATE_SANDBOX_DEFAULT_TIME")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_time),
            default_wall_time: std::env::var("ISOLATE_SANDBOX_DEFAULT_WALL_TIME")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_wall_time),
            default_extra_time: std::env::var("ISOLATE_SANDBOX_DEFAULT_EXTRA_TIME")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_extra_time),
            default_stack: std::env::var("ISOLATE_SANDBOX_DEFAULT_STACK")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_stack),
            default_fsize: std::env::var("ISOLATE_SANDBOX_DEFAULT_FSIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_fsize),
            default_open_files: std::env::var("ISOLATE_SANDBOX_DEFAULT_OPEN_FILES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_open_files),
            default_processes: std::env::var("ISOLATE_SANDBOX_DEFAULT_PROCESSES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(default.default_processes),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub config_dir: PathBuf,
    pub box_pool_size: u32,
    pub api_key: Option<String>,
    pub sandbox: SandboxConfig,
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

        let sandbox = SandboxConfig::from_env();

        Self {
            port,
            config_dir,
            box_pool_size,
            api_key,
            sandbox,
        }
    }
}

