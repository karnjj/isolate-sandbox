use crate::domain::entities::Language;
use crate::domain::error::{DomainError, DomainResult};
use crate::domain::repositories::LanguageRepository;
use crate::external::process::ProcessExecutor;
use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct FileSystemLanguageRepository {
    config_dir: PathBuf,
    languages: Arc<RwLock<HashMap<String, Language>>>,
    process_executor: ProcessExecutor,
}

impl FileSystemLanguageRepository {
    pub fn new(config_dir: PathBuf) -> Self {
        Self {
            config_dir,
            languages: Arc::new(RwLock::new(HashMap::new())),
            process_executor: ProcessExecutor::new(),
        }
    }

    async fn discover_languages(&self) -> DomainResult<Vec<Language>> {
        let mut languages = Vec::new();

        let entries = tokio::fs::read_dir(&self.config_dir)
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to read config dir: {}", e)))?;

        let mut entries = entries;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| DomainError::Internal(format!("Failed to read entry: {}", e)))?
        {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .ok_or_else(|| DomainError::Internal("Invalid directory name".to_string()))?
                    .to_string();

                // Determine file extension based on language name
                let extension = match name.as_str() {
                    "python" => "py",
                    "javascript" => "js",
                    "typescript" => "ts",
                    _ => "txt",
                }
                .to_string();

                let language = Language::new(name, extension, path);
                languages.push(language);
            }
        }

        Ok(languages)
    }
}

#[async_trait]
impl LanguageRepository for FileSystemLanguageRepository {
    async fn find_by_name(&self, name: &str) -> DomainResult<Language> {
        let languages = self.languages.read().await;
        languages
            .get(name)
            .cloned()
            .ok_or_else(|| DomainError::LanguageNotFound(name.to_string()))
    }

    async fn list_all(&self) -> DomainResult<Vec<Language>> {
        let languages = self.languages.read().await;
        Ok(languages.values().cloned().collect())
    }

    async fn setup_all(&self) -> DomainResult<()> {
        // Discover available languages
        let discovered = self.discover_languages().await?;

        log::info!("Found {} language(s)", discovered.len());

        // Setup each language
        for language in &discovered {
            let setup_script = language.setup_script();
            
            if !setup_script.exists() {
                log::warn!(
                    "Setup script not found for {}: {}",
                    language.name,
                    setup_script.display()
                );
                continue;
            }

            log::info!("Running setup for {}...", language.name);
            
            let script_path = setup_script
                .to_str()
                .ok_or_else(|| DomainError::Internal("Invalid script path".to_string()))?;
            
            self.process_executor.execute_script(script_path).await?;
            
            log::info!("Setup completed for {}", language.name);
        }

        // Store languages in cache
        let mut languages = self.languages.write().await;
        for language in discovered {
            languages.insert(language.name.clone(), language);
        }

        Ok(())
    }
}

