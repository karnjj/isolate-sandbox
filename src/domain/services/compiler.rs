use crate::domain::entities::Language;
use crate::domain::error::DomainResult;
use async_trait::async_trait;
use std::path::PathBuf;

#[async_trait]
pub trait CompilerService: Send + Sync {
    async fn compile(
        &self,
        language: &Language,
        source_path: &PathBuf,
        output_dir: &PathBuf,
    ) -> DomainResult<PathBuf>;
}

