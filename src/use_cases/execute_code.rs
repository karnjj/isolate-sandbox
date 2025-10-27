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
        log::debug!("Finding language configuration for: {}", request.language);
        let language = self.language_repo.find_by_name(&request.language).await?;
        log::debug!("Found language: {} with extension: {}", language.name, language.extension);

        // Create temporary directory for this execution
        log::debug!("Creating temporary directory for execution");
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();
        log::debug!("Created temporary directory at: {:?}", temp_path);

        // Write source code to temp directory
        let source_file = temp_path.join(format!("source.{}", language.extension));
        log::debug!("Writing source code to: {:?} ({} bytes)", source_file, request.code.len());
        fs::write(&source_file, &request.code).await?;
        log::debug!("Source code written successfully");

        // Compile the code
        log::debug!("Compiling code for language: {}", language.name);
        let binary_path = self.compiler.compile(&language, &source_file, &temp_path.to_path_buf()).await?;
        log::debug!("Code compiled successfully, binary path: {:?}", binary_path);

        // Acquire box ID from pool
        log::debug!("Acquiring box ID from pool");
        let box_id = self.sandbox.acquire_box_id().await?;
        log::debug!("Acquired box ID: {}", box_id);

        // Execute in sandbox
        let metadata_path = temp_path.join(format!("meta-{}.txt", box_id));
        log::debug!("Configuring sandbox execution for box ID: {}", box_id);
        let config = SandboxExecutionConfig {
            box_id,
            binary_path,
            runner_path: language.runner_path(),
            language: language.clone(),
            metadata_path,
        };

        log::debug!("Executing code in sandbox with box ID: {}", box_id);
        let sandbox_result = self.sandbox.execute(config).await?;
        log::debug!("Sandbox execution completed for box ID: {}", box_id);

        // Note: Box ID is NOT released here anymore - it must be explicitly cleaned up
        // via the cleanup endpoint to allow file inspection after execution

        log::debug!("Returning execution result with box ID: {}", sandbox_result.box_id);
        Ok(ExecutionResult {
            stdout: sandbox_result.stdout,
            stderr: sandbox_result.stderr,
            metadata: sandbox_result.metadata,
            box_id: sandbox_result.box_id,
        })
    }
}

