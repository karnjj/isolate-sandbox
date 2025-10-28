use crate::domain::entities::ExecutionMetadata;
use crate::domain::error::{DomainError, DomainResult};
use crate::domain::services::{SandboxExecutionConfig, SandboxExecutionResult, SandboxService};
use crate::external::file_system::FileSystem;
use crate::external::process::ProcessExecutor;
use crate::external::services::box_pool::BoxPool;
use async_trait::async_trait;
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;

pub struct IsolateSandboxService {
    box_pool: Arc<BoxPool>,
    process_executor: ProcessExecutor,
    file_system: FileSystem,
}

impl IsolateSandboxService {
    pub fn new(pool_size: u32) -> Self {
        Self {
            box_pool: Arc::new(BoxPool::new(pool_size)),
            process_executor: ProcessExecutor::new(),
            file_system: FileSystem::new(),
        }
    }

    async fn init_sandbox(&self, box_id: u32) -> DomainResult<()> {
        let box_id_str = box_id.to_string();
        let (_, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["isolate", "-b", &box_id_str, "--cg", "--init"])
            .await?;

        if exit_code != 0 {
            return Err(DomainError::SandboxError(format!(
                "Failed to initialize sandbox: {}",
                stderr
            )));
        }

        Ok(())
    }

    async fn cleanup_sandbox(&self, box_id: u32) -> DomainResult<()> {
        let box_id_str = box_id.to_string();
        let (_, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["isolate", "-b", &box_id_str, "--cg", "--cleanup"])
            .await?;

        if exit_code != 0 {
            log::warn!("Failed to cleanup sandbox {}: {}", box_id, stderr);
        }

        Ok(())
    }

    async fn copy_to_sandbox(
        &self,
        box_id: u32,
        source: &PathBuf,
        dest_name: &str,
    ) -> DomainResult<()> {
        let sandbox_path = PathBuf::from(format!("/var/lib/isolate/{}/box/{}", box_id, dest_name));
        
        let source_str = source
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid source path".to_string()))?;
        let dest_str = sandbox_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid dest path".to_string()))?;

        let (_, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["cp", source_str, dest_str])
            .await?;

        if exit_code != 0 {
            return Err(DomainError::SandboxError(format!(
                "Failed to copy file to sandbox: {}",
                stderr
            )));
        }

        Ok(())
    }

    fn get_site_packages_path(&self, language_name: &str) -> DomainResult<String> {
        let home_dir = std::env::var("HOME")
            .map_err(|_| DomainError::Internal("HOME env var not set".to_string()))?;
        
        let env_path = format!("{}/.isolate-sandbox/environment/{}", home_dir, language_name);
        
        // For python, find site-packages
        if language_name == "python" {
            let lib_path = PathBuf::from(&env_path).join("lib");
            if lib_path.exists() {
                // Find the python version directory
                if let Ok(entries) = std::fs::read_dir(&lib_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            let site_packages = path.join("site-packages");
                            if site_packages.exists() {
                                return Ok(site_packages
                                    .to_str()
                                    .ok_or_else(|| DomainError::Internal("Invalid path".to_string()))?
                                    .to_string());
                            }
                        }
                    }
                }
            }
        }
        
        Err(DomainError::Internal(format!(
            "Could not find site-packages for {}",
            language_name
        )))
    }

    async fn run_in_sandbox(
        &self,
        box_id: u32,
        site_packages: &str,
        metadata_path: &PathBuf,
    ) -> DomainResult<(String, String, i32)> {
        let box_id_str = box_id.to_string();
        let meta_path_str = metadata_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid metadata path".to_string()))?;

        let packages_arg = format!("--dir=/packages={}", site_packages);
        let meta_arg = format!("--meta={}", meta_path_str);

        let args = vec![
            "isolate",
            "-b",
            &box_id_str,
            "--cg",
            // "--cg-mem=524288", // 512MB
            // "--mem=512000", // 512MB
            // "--time=30", // 30 seconds
            // "--wall-time=60", // 60 seconds
            // "--extra-time=10", // 10 seconds
            // "--stack=128000", // 128KB
            // "--fsize=102400", // 100MB
            "--open-files=0", // unlimited files
            "--processes", // unlimited processes
            &packages_arg,
            "--env=HOME=/box",
            "--env=PYTHONPATH=/packages",
            &meta_arg,
            "--run",
            "--",
            "runner",
        ];

        self.process_executor
            .execute_command("sudo", &args)
            .await
    }

    fn parse_metadata(&self, metadata_content: &str) -> ExecutionMetadata {
        let mut metadata = ExecutionMetadata::new();

        let time_regex = Regex::new(r"time:([\d.]+)").unwrap();
        let time_wall_regex = Regex::new(r"time-wall:([\d.]+)").unwrap();
        let memory_regex = Regex::new(r"cg-mem:(\d+)").unwrap();
        let status_regex = Regex::new(r"status:(\w+)").unwrap();
        let exitcode_regex = Regex::new(r"exitcode:(\d+)").unwrap();

        if let Some(cap) = time_regex.captures(metadata_content) {
            metadata.time = cap[1].parse().unwrap_or(0.0);
        }

        if let Some(cap) = time_wall_regex.captures(metadata_content) {
            metadata.time_wall = cap[1].parse().unwrap_or(0.0);
        }

        if let Some(cap) = memory_regex.captures(metadata_content) {
            metadata.memory = cap[1].parse().unwrap_or(0);
        }

        if let Some(cap) = status_regex.captures(metadata_content) {
            metadata.status = cap[1].to_string();
        }

        if let Some(cap) = exitcode_regex.captures(metadata_content) {
            metadata.exit_code = cap[1].parse().unwrap_or(0);
        }

        metadata
    }
}

#[async_trait]
impl SandboxService for IsolateSandboxService {
    async fn execute(&self, config: SandboxExecutionConfig) -> DomainResult<SandboxExecutionResult> {
        // Initialize sandbox
        log::debug!("Initializing sandbox for box ID: {}", config.box_id);
        self.init_sandbox(config.box_id).await?;
        log::debug!("Sandbox initialized successfully for box ID: {}", config.box_id);

        // Copy binary and runner to sandbox
        log::debug!("Copying binary to sandbox: {:?}", config.binary_path);
        self.copy_to_sandbox(config.box_id, &config.binary_path, "bin")
            .await?;
        log::debug!("Binary copied successfully");

        log::debug!("Copying runner to sandbox: {:?}", config.runner_path);
        self.copy_to_sandbox(config.box_id, &config.runner_path, "runner")
            .await?;
        log::debug!("Runner copied successfully");

        // Get site packages path
        log::debug!("Getting site packages path for language: {}", config.language.name);
        let site_packages = self.get_site_packages_path(&config.language.name)?;
        log::debug!("Found site packages path: {}", site_packages);

        // Execute in sandbox
        log::debug!("Running code in sandbox with box ID: {}", config.box_id);
        let (stdout, stderr, _exit_code) = self
            .run_in_sandbox(config.box_id, &site_packages, &config.metadata_path)
            .await?;
        log::debug!("Code execution completed in sandbox");

        // Parse metadata
        log::debug!("Parsing execution metadata from: {:?}", config.metadata_path);
        let metadata_content = self.file_system.read_to_string(&config.metadata_path).await?;
        let metadata = self.parse_metadata(&metadata_content);
        log::debug!("Metadata parsed: time={}s, time_wall={}s, memory={}KB, status={}", 
                   metadata.time, metadata.time_wall, metadata.memory, metadata.status);

        // Clean up runtime files (bin and runner) while preserving user-created files
        log::debug!("Cleaning up runtime files in sandbox");
        self.delete_file(config.box_id, "bin").await?;
        self.delete_file(config.box_id, "runner").await?;
        log::debug!("Runtime files cleaned up");

        // Note: Full sandbox cleanup must be called explicitly via cleanup endpoint

        log::debug!("Sandbox execution result ready with {} bytes stdout, {} bytes stderr", 
                   stdout.len(), stderr.len());
        Ok(SandboxExecutionResult {
            stdout,
            stderr,
            metadata,
            box_id: config.box_id,
        })
    }

    async fn acquire_box_id(&self) -> DomainResult<u32> {
        self.box_pool.acquire().await
    }

    async fn release_box_id(&self, box_id: u32) -> DomainResult<()> {
        self.box_pool.release(box_id).await
    }

    async fn list_files(&self, box_id: u32) -> DomainResult<Vec<String>> {
        let box_path = PathBuf::from(format!("/var/lib/isolate/{}/box", box_id));
        
        let box_path_str = box_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid box path".to_string()))?;

        let (stdout, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["ls", "-1", box_path_str])
            .await?;

        if exit_code != 0 {
            return Err(DomainError::SandboxError(format!(
                "Failed to list files in box: {}",
                stderr
            )));
        }

        let files: Vec<String> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        Ok(files)
    }

    async fn get_file_base64(&self, box_id: u32, filename: &str) -> DomainResult<String> {
        let file_path = PathBuf::from(format!("/var/lib/isolate/{}/box/{}", box_id, filename));
        
        let file_path_str = file_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid file path".to_string()))?;

        let (base64_content, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["base64", "-w", "0", file_path_str])
            .await?;

        if exit_code != 0 {
            return Err(DomainError::SandboxError(format!(
                "Failed to read file from box: {}",
                stderr
            )));
        }

        Ok(base64_content)
    }

    async fn delete_file(&self, box_id: u32, filename: &str) -> DomainResult<()> {
        let file_path = PathBuf::from(format!("/var/lib/isolate/{}/box/{}", box_id, filename));
        
        let file_path_str = file_path
            .to_str()
            .ok_or_else(|| DomainError::Internal("Invalid file path".to_string()))?;

        let (_, stderr, exit_code) = self
            .process_executor
            .execute_command("sudo", &["rm", "-f", file_path_str])
            .await?;

        if exit_code != 0 {
            log::warn!("Failed to delete file {} from box {}: {}", filename, box_id, stderr);
        }

        Ok(())
    }

    async fn cleanup(&self, box_id: u32) -> DomainResult<()> {
        self.cleanup_sandbox(box_id).await
    }
}

