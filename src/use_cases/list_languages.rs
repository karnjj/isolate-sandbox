use crate::domain::entities::Language;
use crate::domain::error::DomainResult;
use crate::domain::repositories::LanguageRepository;
use std::sync::Arc;

pub struct ListLanguagesUseCase {
    language_repo: Arc<dyn LanguageRepository>,
}

impl ListLanguagesUseCase {
    pub fn new(language_repo: Arc<dyn LanguageRepository>) -> Self {
        Self { language_repo }
    }

    pub async fn execute(&self) -> DomainResult<Vec<Language>> {
        self.language_repo.list_all().await
    }
}

