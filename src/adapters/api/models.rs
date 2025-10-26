use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ExecuteRequest {
    /// Programming language to execute (e.g., "python", "rust", "javascript")
    pub language: String,
    /// Source code to execute
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExecuteResponse {
    /// Standard output from the execution
    pub stdout: String,
    /// Standard error from the execution
    pub stderr: String,
    /// Execution metadata
    pub metadata: MetadataResponse,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MetadataResponse {
    /// Execution time in seconds
    pub time: f64,
    /// Wall clock time in seconds
    pub time_wall: f64,
    /// Memory usage in bytes
    pub memory: u64,
    /// Exit code of the process
    pub exit_code: i32,
    /// Execution status
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    /// Health status ("ok" or "error")
    pub status: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LanguagesResponse {
    /// List of supported programming languages
    pub languages: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
}

