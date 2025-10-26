use crate::domain::entities::Language;
use crate::domain::error::DomainResult;
use async_trait::async_trait;

#[async_trait]
pub trait LanguageRepository: Send + Sync {
    async fn find_by_name(&self, name: &str) -> DomainResult<Language>;
    async fn list_all(&self) -> DomainResult<Vec<Language>>;
    async fn setup_all(&self) -> DomainResult<()>;
}

