use crate::adapters::api::error::ApiError;
use crate::adapters::api::models::{
    BoxFileResponse, BoxFilesResponse, CleanupResponse, ExecuteRequest, ExecuteResponse,
    HealthResponse, LanguagesResponse, MetadataResponse,
};
use crate::domain::entities::ExecutionRequest as DomainExecutionRequest;
use crate::use_cases::{
    CleanupBoxUseCase, ExecuteCodeUseCase, GetBoxFileUseCase, HealthCheckUseCase,
    ListBoxFilesUseCase, ListLanguagesUseCase,
};
use axum::extract::{Path, State};
use axum::Json;
use std::sync::Arc;

pub struct AppState {
    pub execute_code_use_case: Arc<ExecuteCodeUseCase>,
    pub list_languages_use_case: Arc<ListLanguagesUseCase>,
    pub health_check_use_case: Arc<HealthCheckUseCase>,
    pub list_box_files_use_case: Arc<ListBoxFilesUseCase>,
    pub get_box_file_use_case: Arc<GetBoxFileUseCase>,
    pub cleanup_box_use_case: Arc<CleanupBoxUseCase>,
    pub api_key: Option<String>,
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
        box_id: result.box_id,
    }))
}

/// List files in a sandbox box
///
/// Returns a list of files in the specified sandbox box
#[utoipa::path(
    get,
    path = "/boxes/{box_id}/files",
    params(
        ("box_id" = u32, Path, description = "Box ID to list files from")
    ),
    responses(
        (status = 200, description = "List of files in the box", body = BoxFilesResponse),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Box Management"
)]
pub async fn list_box_files_handler(
    State(state): State<Arc<AppState>>,
    Path(box_id): Path<u32>,
) -> Result<Json<BoxFilesResponse>, ApiError> {
    let files = state.list_box_files_use_case.execute(box_id).await?;

    Ok(Json(BoxFilesResponse { files }))
}

/// Get a file from a sandbox box
///
/// Returns the content of a specific file from the sandbox box
#[utoipa::path(
    get,
    path = "/boxes/{box_id}/files/{filename}",
    params(
        ("box_id" = u32, Path, description = "Box ID to get file from"),
        ("filename" = String, Path, description = "Name of the file to retrieve")
    ),
    responses(
        (status = 200, description = "File content (base64 encoded)", body = BoxFileResponse),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Box Management"
)]
pub async fn get_box_file_handler(
    State(state): State<Arc<AppState>>,
    Path((box_id, filename)): Path<(u32, String)>,
) -> Result<Json<BoxFileResponse>, ApiError> {
    let content_bytes = state
        .get_box_file_use_case
        .execute(box_id, &filename)
        .await?;

    let content = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, content_bytes);

    Ok(Json(BoxFileResponse { content, filename }))
}

/// Cleanup a sandbox box
///
/// Cleans up the specified sandbox box and releases it back to the pool
#[utoipa::path(
    delete,
    path = "/boxes/{box_id}",
    params(
        ("box_id" = u32, Path, description = "Box ID to cleanup")
    ),
    responses(
        (status = 200, description = "Box cleaned up successfully", body = CleanupResponse),
        (status = 500, description = "Internal server error"),
    ),
    tag = "Box Management"
)]
pub async fn cleanup_box_handler(
    State(state): State<Arc<AppState>>,
    Path(box_id): Path<u32>,
) -> Result<Json<CleanupResponse>, ApiError> {
    state.cleanup_box_use_case.execute(box_id).await?;

    Ok(Json(CleanupResponse {
        message: format!("Box {} cleaned up successfully", box_id),
    }))
}

