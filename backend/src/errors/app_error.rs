use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("App failed to start up")]
    StartupError,
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
    #[error("{0}")]
    RequestError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (message, status_code) = match self {
            AppError::StartupError => (
                "internal server error".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AppError::EnvError(err) => (err, StatusCode::INTERNAL_SERVER_ERROR),
            AppError::OperationFailed(err) => (err, StatusCode::INTERNAL_SERVER_ERROR),
            AppError::RequestError(err) => (err, StatusCode::UNPROCESSABLE_ENTITY),
        };

        (status_code, message).into_response()
    }
}
