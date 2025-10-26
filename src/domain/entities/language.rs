use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub extension: String,
    pub config_dir: PathBuf,
}

impl Language {
    pub fn new(name: String, extension: String, config_dir: PathBuf) -> Self {
        Self {
            name,
            extension,
            config_dir,
        }
    }

    pub fn setup_script(&self) -> PathBuf {
        self.config_dir.join("setup.sh")
    }

    pub fn compiler_path(&self) -> PathBuf {
        self.config_dir.join("compiler")
    }

    pub fn runner_path(&self) -> PathBuf {
        self.config_dir.join("runner")
    }
}

