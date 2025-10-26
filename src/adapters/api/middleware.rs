use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Middleware to check API key authentication
///
/// Returns 403 if X-API-Key header is missing
/// Returns 401 if X-API-Key header is present but invalid
pub async fn auth_middleware(
    expected_api_key: Option<String>,
    request: Request,
    next: Next,
) -> Response {
    // If no API key is configured, allow all requests
    let Some(expected_key) = expected_api_key else {
        return next.run(request).await;
    };

    // Check if X-API-Key header is present
    let api_key_header = request.headers().get("x-api-key");

    match api_key_header {
        None => {
            // Header is missing - return 403
            (
                StatusCode::FORBIDDEN,
                Json(json!({
                    "error": "Missing X-API-Key header"
                })),
            )
                .into_response()
        }
        Some(key) => {
            // Header is present - validate it
            match key.to_str() {
                Ok(key_str) if key_str == expected_key => {
                    // Valid API key - proceed
                    next.run(request).await
                }
                _ => {
                    // Invalid API key - return 401
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "Invalid API key"
                        })),
                    )
                        .into_response()
                }
            }
        }
    }
}

