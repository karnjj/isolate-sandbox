use crate::adapters::api::handlers::{
    cleanup_box_handler, execute_code_handler, get_box_file_handler, health_handler,
    list_box_files_handler, list_languages_handler, AppState,
};
use crate::adapters::api::middleware::auth_middleware;
use crate::adapters::api::models::{
    BoxFileResponse, BoxFilesResponse, CleanupResponse, ErrorResponse, ExecuteRequest,
    ExecuteResponse, HealthResponse, LanguagesResponse, MetadataResponse,
};
use axum::{middleware, routing::delete, routing::get, routing::post, Router};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::adapters::api::handlers::health_handler,
        crate::adapters::api::handlers::list_languages_handler,
        crate::adapters::api::handlers::execute_code_handler,
        crate::adapters::api::handlers::list_box_files_handler,
        crate::adapters::api::handlers::get_box_file_handler,
        crate::adapters::api::handlers::cleanup_box_handler,
    ),
    components(
        schemas(
            HealthResponse,
            LanguagesResponse,
            ExecuteRequest,
            ExecuteResponse,
            MetadataResponse,
            BoxFilesResponse,
            BoxFileResponse,
            CleanupResponse,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Languages", description = "Language management endpoints"),
        (name = "Execution", description = "Code execution endpoints"),
        (name = "Box Management", description = "Sandbox box file management and cleanup endpoints")
    ),
    info(
        title = "Isolate Sandbox API",
        version = "0.1.0",
        description = "A secure sandboxed code execution service using Linux Isolate",
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header(
                        utoipa::openapi::security::ApiKeyValue::new("X-API-Key")
                    )
                )
            )
        }
    }
}

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_key = state.api_key.clone();

    // Protected routes that require authentication
    let protected_routes = Router::new()
        .route("/languages", get(list_languages_handler))
        .route("/execute", post(execute_code_handler))
        .route("/boxes/:box_id/files", get(list_box_files_handler))
        .route("/boxes/:box_id/files/:filename", get(get_box_file_handler))
        .route("/boxes/:box_id", delete(cleanup_box_handler))
        .layer(middleware::from_fn(move |request, next| {
            let api_key = api_key.clone();
            async move { auth_middleware(api_key, request, next).await }
        }));

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_handler))
        .merge(protected_routes)
        .with_state(state)
}

