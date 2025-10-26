use crate::adapters::api::handlers::{
    execute_code_handler, health_handler, list_languages_handler, AppState,
};
use crate::adapters::api::models::{
    ExecuteRequest, ExecuteResponse, HealthResponse, LanguagesResponse, MetadataResponse,
    ErrorResponse,
};
use axum::{routing::get, routing::post, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::api::handlers::health_handler,
        crate::adapters::api::handlers::list_languages_handler,
        crate::adapters::api::handlers::execute_code_handler,
    ),
    components(
        schemas(
            HealthResponse,
            LanguagesResponse,
            ExecuteRequest,
            ExecuteResponse,
            MetadataResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Languages", description = "Language management endpoints"),
        (name = "Execution", description = "Code execution endpoints")
    ),
    info(
        title = "Isolate Sandbox API",
        version = "0.1.0",
        description = "A secure sandboxed code execution service using Linux Isolate",
    )
)]
pub struct ApiDoc;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_handler))
        .route("/languages", get(list_languages_handler))
        .route("/execute", post(execute_code_handler))
        .with_state(state)
}

