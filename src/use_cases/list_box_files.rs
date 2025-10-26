use crate::domain::error::DomainResult;
use crate::domain::services::SandboxService;
use std::sync::Arc;

pub struct ListBoxFilesUseCase {
    sandbox: Arc<dyn SandboxService>,
}

impl ListBoxFilesUseCase {
    pub fn new(sandbox: Arc<dyn SandboxService>) -> Self {
        Self { sandbox }
    }

    pub async fn execute(&self, box_id: u32) -> DomainResult<Vec<String>> {
        self.sandbox.list_files(box_id).await
    }
}

