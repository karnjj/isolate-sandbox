use crate::domain::entities::Language;
use crate::domain::error::{DomainError, DomainResult};
use crate::domain::services::CompilerService;
use crate::external::process::ProcessExecutor;
use async_trait::async_trait;
use std::path::PathBuf;

pub struct CompilerServiceImpl {
    process_executor: ProcessExecutor,
}

impl CompilerServiceImpl {
    pub fn new() -> Self {
        Self {
            process_executor: ProcessExecutor::new(),
        }
    }

    fn sanitize_stderr(&self, stderr: &str, source_path: &PathBuf) -> String {
        let source_str = source_path.to_str().unwrap_or("");
        
        // Replace file paths with *** in error messages, keeping only the extension
        if let Some(extension) = source_path.extension() {
            let replacement = format!("*******.{}", extension.to_string_lossy());
            stderr.replace(source_str, &replacement)
        } else {
            stderr.replace(source_str, "*******")
        }
    }
}

impl Default for CompilerServiceImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CompilerService for CompilerServiceImpl {
    async fn compile(
        &self,
        language: &Language,
        source_path: &PathBuf,
        output_dir: &PathBuf,
    ) -> DomainResult<PathBuf> {
        let compiler_path = language.compiler_path();
        
        if !compiler_path.exists() {
            return Err(DomainError::CompilationFailed(format!(
                "Compiler not found: {}",
                compiler_path.display()
            )));
        }

        let source_str = source_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid source path".to_string()))?;

        let (stdout, stderr, exit_code) = self
            .process_executor
            .execute_command(
                compiler_path.to_str().unwrap(),
                &[source_str],
            )
            .await?;

        if exit_code != 0 {
            let sanitized_stderr = self.sanitize_stderr(&stderr, source_path);
            return Err(DomainError::CompilationFailed(format!(
                "Compilation failed with exit code {}: {}",
                exit_code, sanitized_stderr
            )));
        }

        // The compiler script outputs the compiled binary to output_dir/bin
        let binary_path = output_dir.join("bin");
        
        if !binary_path.exists() {
            return Err(DomainError::CompilationFailed(format!(
                "Compiled binary not found at: {}. Stdout: {}, Stderr: {}",
                binary_path.display(),
                stdout,
                stderr
            )));
        }

        Ok(binary_path)
    }
}

