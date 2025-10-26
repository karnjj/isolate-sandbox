use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub metadata: ExecutionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub time: f64,
    pub time_wall: f64,
    pub memory: u64,
    pub exit_code: i32,
    pub status: String,
}

impl ExecutionMetadata {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            time_wall: 0.0,
            memory: 0,
            exit_code: 0,
            status: "OK".to_string(),
        }
    }
}

impl Default for ExecutionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

