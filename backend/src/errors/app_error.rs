#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up")]
    StartupError,
    #[error("{0}")]
    OperationFailed(String),
}
