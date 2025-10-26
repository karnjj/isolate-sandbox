use crate::adapters::api::models::ErrorResponse;
use crate::domain::error::DomainError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        Self(error)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            DomainError::LanguageNotFound(ref msg) => (StatusCode::NOT_FOUND, msg.clone()),
            DomainError::CompilationFailed(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            DomainError::ExecutionFailed(ref msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            DomainError::BoxPoolExhausted => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service is busy, please try again later".to_string(),
            ),
            DomainError::InvalidConfiguration(ref msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            DomainError::SandboxError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            DomainError::IoError(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            DomainError::Internal(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = Json(ErrorResponse { error: message });

        (status, body).into_response()
    }
}

