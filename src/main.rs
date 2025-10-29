mod adapters;
mod config;
mod domain;
mod external;
mod use_cases;

use adapters::api::{create_router, AppState};
use config::Config;
use domain::repositories::LanguageRepository;
use external::repositories::FileSystemLanguageRepository;
use external::services::{CompilerServiceImpl, IsolateSandboxService};
use use_cases::{
    CleanupBoxUseCase, ExecuteCodeUseCase, GetBoxFileUseCase, HealthCheckUseCase,
    ListBoxFilesUseCase, ListLanguagesUseCase,
};

use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logger
    env_logger::init();

    log::info!("Starting Isolate Sandbox Service...");

    // Load configuration
    let config = Config::from_env();
    log::info!("Configuration loaded: {:?}", config);

    // Verify isolate is installed
    log::info!("Verifying isolate installation...");
    verify_isolate().await?;

    // Initialize repositories
    log::info!("Initializing language repository...");
    let language_repo = Arc::new(FileSystemLanguageRepository::new(config.config_dir.clone()));

    // Setup all languages
    log::info!("Setting up languages...");
    language_repo.setup_all().await?;

    let languages = language_repo.list_all().await?;
    log::info!(
        "Available languages: {}",
        languages
            .iter()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    // Initialize services
    log::info!("Initializing services...");
    let compiler_service = Arc::new(CompilerServiceImpl::new());
    let sandbox_service = Arc::new(IsolateSandboxService::new(config.box_pool_size, config.sandbox));

    // Initialize use cases
    log::info!("Initializing use cases...");
    let execute_code_use_case = Arc::new(ExecuteCodeUseCase::new(
        language_repo.clone(),
        compiler_service,
        sandbox_service.clone(),
    ));
    let list_languages_use_case = Arc::new(ListLanguagesUseCase::new(language_repo));
    let health_check_use_case = Arc::new(HealthCheckUseCase::new());
    let list_box_files_use_case = Arc::new(ListBoxFilesUseCase::new(sandbox_service.clone()));
    let get_box_file_use_case = Arc::new(GetBoxFileUseCase::new(sandbox_service.clone()));
    let cleanup_box_use_case = Arc::new(CleanupBoxUseCase::new(sandbox_service));

    // Create app state
    let app_state = Arc::new(AppState {
        execute_code_use_case,
        list_languages_use_case,
        health_check_use_case,
        list_box_files_use_case,
        get_box_file_use_case,
        cleanup_box_use_case,
        api_key: config.api_key.clone(),
    });

    // Create router
    let app = create_router(app_state);

    // Start server
    let addr = format!("0.0.0.0:{}", config.port);
    log::info!("Server listening on {}", addr);
    log::info!("Swagger UI available at http://localhost:{}/swagger-ui", config.port);
    log::info!("OpenAPI spec available at http://localhost:{}/api-docs/openapi.json", config.port);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn verify_isolate() -> anyhow::Result<()> {
    let output = tokio::process::Command::new("sudo")
        .arg("isolate")
        .arg("--version")
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            log::info!("Isolate is installed and accessible");
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Isolate command failed: {}", stderr)
        }
        Err(e) => {
            anyhow::bail!("Isolate not found. Please install isolate: {}", e)
        }
    }
}

