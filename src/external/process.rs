use crate::domain::error::{DomainError, DomainResult};
use std::process::Stdio;
use tokio::process::Command;

pub struct ProcessExecutor;

impl ProcessExecutor {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute_command(
        &self,
        program: &str,
        args: &[&str],
    ) -> DomainResult<(String, String, i32)> {
        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to execute command: {}", e)))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, exit_code))
    }

    /// Execute a command and return stdout as binary data (Vec<u8>)
    /// This is useful for reading binary files like images, PDFs, etc.
    pub async fn execute_command_binary(
        &self,
        program: &str,
        args: &[&str],
    ) -> DomainResult<(Vec<u8>, String, i32)> {
        let output = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to execute command: {}", e)))?;

        let stdout = output.stdout; // Keep as raw bytes without UTF-8 conversion
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok((stdout, stderr, exit_code))
    }

    pub async fn execute_script(&self, script_path: &str) -> DomainResult<()> {
        let output = Command::new("bash")
            .arg(script_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to execute script: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(DomainError::Internal(format!(
                "Script execution failed: {}",
                stderr
            )));
        }

        Ok(())
    }
}

impl Default for ProcessExecutor {
    fn default() -> Self {
        Self::new()
    }
}

