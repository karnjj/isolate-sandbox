use crate::domain::entities::{ExecutionMetadata, Language};
use crate::domain::error::DomainResult;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct SandboxExecutionConfig {
    pub box_id: u32,
    pub binary_path: PathBuf,
    pub runner_path: PathBuf,
    pub language: Language,
    pub metadata_path: PathBuf,
}

pub struct SandboxExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub metadata: ExecutionMetadata,
    pub box_id: u32,
}

#[async_trait]
pub trait SandboxService: Send + Sync {
    async fn execute(&self, config: SandboxExecutionConfig) -> DomainResult<SandboxExecutionResult>;
    async fn acquire_box_id(&self) -> DomainResult<u32>;
    async fn release_box_id(&self, box_id: u32) -> DomainResult<()>;
    async fn list_files(&self, box_id: u32) -> DomainResult<Vec<String>>;
    async fn get_file(&self, box_id: u32, filename: &str) -> DomainResult<Vec<u8>>;
    async fn cleanup(&self, box_id: u32) -> DomainResult<()>;
}

