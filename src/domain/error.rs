use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Language not found: {0}")]
    LanguageNotFound(String),

    #[error("Compilation failed: {0}")]
    CompilationFailed(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Sandbox error: {0}")]
    SandboxError(String),

    #[error("Box pool exhausted")]
    BoxPoolExhausted,

    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type DomainResult<T> = Result<T, DomainError>;

