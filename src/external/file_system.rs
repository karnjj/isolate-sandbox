use crate::domain::error::{DomainError, DomainResult};
use std::path::PathBuf;
use tokio::fs;

pub struct FileSystem;

impl FileSystem {
    pub fn new() -> Self {
        Self
    }

    pub async fn copy_file(&self, from: &PathBuf, to: &PathBuf) -> DomainResult<()> {
        fs::copy(from, to)
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to copy file: {}", e)))?;
        Ok(())
    }

    pub async fn read_to_string(&self, path: &PathBuf) -> DomainResult<String> {
        fs::read_to_string(path)
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to read file: {}", e)))
    }

    pub async fn ensure_directory_exists(&self, path: &PathBuf) -> DomainResult<()> {
        if !path.exists() {
            fs::create_dir_all(path)
                .await
                .map_err(|e| DomainError::Internal(format!("Failed to create directory: {}", e)))?;
        }
        Ok(())
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

