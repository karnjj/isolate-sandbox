use crate::adapters::api::error::ApiError;
use crate::adapters::api::models::{
    ExecuteRequest, ExecuteResponse, HealthResponse, LanguagesResponse, MetadataResponse,
};
use crate::domain::entities::ExecutionRequest as DomainExecutionRequest;
use crate::use_cases::{ExecuteCodeUseCase, HealthCheckUseCase, ListLanguagesUseCase};
use axum::extract::State;
use axum::Json;
use std::sync::Arc;

pub struct AppState {
    pub execute_code_use_case: Arc<ExecuteCodeUseCase>,
    pub list_languages_use_case: Arc<ListLanguagesUseCase>,
    pub health_check_use_case: Arc<HealthCheckUseCase>,
}

/// Health check endpoint
///
/// Returns the health status of the service
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse),
    ),
    tag = "Health"
)]
pub async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthResponse>, ApiError> {
    let is_healthy = state.health_check_use_case.execute().await;

    Ok(Json(HealthResponse {
        status: if is_healthy { "ok" } else { "error" }.to_string(),
    }))
}

/// List available programming languages
///
/// Returns a list of all supported programming languages
#[utoipa::path(
    get,
    path = "/languages",
    responses(
        (status = 200, description = "List of supported languages", body = LanguagesResponse),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Languages"
)]
pub async fn list_languages_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<LanguagesResponse>, ApiError> {
    let languages = state.list_languages_use_case.execute().await?;

    Ok(Json(LanguagesResponse {
        languages: languages.into_iter().map(|l| l.name).collect(),
    }))
}

/// Execute code in a sandboxed environment
///
/// Executes the provided code in the specified language within an isolated sandbox
#[utoipa::path(
    post,
    path = "/execute",
    request_body = ExecuteRequest,
    responses(
        (status = 200, description = "Code executed successfully", body = ExecuteResponse),
        (status = 400, description = "Invalid request"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Execution"
)]
pub async fn execute_code_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ExecuteRequest>,
) -> Result<Json<ExecuteResponse>, ApiError> {
    let domain_request = DomainExecutionRequest {
        language: request.language,
        code: request.code,
    };

    let result = state.execute_code_use_case.execute(domain_request).await?;

    Ok(Json(ExecuteResponse {
        stdout: result.stdout,
        stderr: result.stderr,
        metadata: MetadataResponse {
            time: result.metadata.time,
            time_wall: result.metadata.time_wall,
            memory: result.metadata.memory,
            exit_code: result.metadata.exit_code,
            status: result.metadata.status,
        },
    }))
}

