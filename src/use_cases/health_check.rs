pub struct HealthCheckUseCase;

impl HealthCheckUseCase {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(&self) -> bool {
        true
    }
}

impl Default for HealthCheckUseCase {
    fn default() -> Self {
        Self::new()
    }
}

