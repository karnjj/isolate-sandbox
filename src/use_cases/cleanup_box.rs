use crate::domain::error::DomainResult;
use crate::domain::services::SandboxService;
use std::sync::Arc;

pub struct CleanupBoxUseCase {
    sandbox: Arc<dyn SandboxService>,
}

impl CleanupBoxUseCase {
    pub fn new(sandbox: Arc<dyn SandboxService>) -> Self {
        Self { sandbox }
    }

    pub async fn execute(&self, box_id: u32) -> DomainResult<()> {
        // Cleanup the sandbox
        self.sandbox.cleanup(box_id).await?;
        
        // Release box ID back to pool
        self.sandbox.release_box_id(box_id).await?;
        
        Ok(())
    }
}

