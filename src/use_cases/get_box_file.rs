use crate::domain::error::DomainResult;
use crate::domain::services::SandboxService;
use std::sync::Arc;

pub struct GetBoxFileUseCase {
    sandbox: Arc<dyn SandboxService>,
}

impl GetBoxFileUseCase {
    pub fn new(sandbox: Arc<dyn SandboxService>) -> Self {
        Self { sandbox }
    }

    pub async fn execute(&self, box_id: u32, filename: &str) -> DomainResult<Vec<u8>> {
        self.sandbox.get_file(box_id, filename).await
    }
}

