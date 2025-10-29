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

