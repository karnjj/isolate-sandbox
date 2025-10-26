use crate::domain::entities::{ExecutionRequest, ExecutionResult};
use crate::domain::error::DomainResult;
use crate::domain::repositories::LanguageRepository;
use crate::domain::services::{CompilerService, SandboxExecutionConfig, SandboxService};
use std::sync::Arc;
use tempfile::TempDir;
use tokio::fs;

pub struct ExecuteCodeUseCase {
    language_repo: Arc<dyn LanguageRepository>,
    compiler: Arc<dyn CompilerService>,
    sandbox: Arc<dyn SandboxService>,
}

impl ExecuteCodeUseCase {
    pub fn new(
        language_repo: Arc<dyn LanguageRepository>,
        compiler: Arc<dyn CompilerService>,
        sandbox: Arc<dyn SandboxService>,
    ) -> Self {
        Self {
            language_repo,
            compiler,
            sandbox,
        }
    }

    pub async fn execute(&self, request: ExecutionRequest) -> DomainResult<ExecutionResult> {
        // Find language configuration
        let language = self.language_repo.find_by_name(&request.language).await?;

        // Create temporary directory for this execution
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Write source code to temp directory
        let source_file = temp_path.join(format!("source.{}", language.extension));
        fs::write(&source_file, &request.code).await?;

        // Compile the code
        let binary_path = self.compiler.compile(&language, &source_file, &temp_path.to_path_buf()).await?;

        // Acquire box ID from pool
        let box_id = self.sandbox.acquire_box_id().await?;

        // Execute in sandbox
        let metadata_path = temp_path.join(format!("meta-{}.txt", box_id));
        let config = SandboxExecutionConfig {
            box_id,
            binary_path,
            runner_path: language.runner_path(),
            language: language.clone(),
            metadata_path,
        };

        let sandbox_result = self.sandbox.execute(config).await?;

        // Note: Box ID is NOT released here anymore - it must be explicitly cleaned up
        // via the cleanup endpoint to allow file inspection after execution

        Ok(ExecutionResult {
            stdout: sandbox_result.stdout,
            stderr: sandbox_result.stderr,
            metadata: sandbox_result.metadata,
            box_id: sandbox_result.box_id,
        })
    }
}

