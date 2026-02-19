// Responsibility: Define centralized error surfaces and HTTP translation for the entire application.
// Design reasoning: Using a single error enum lets layers bubble issues while preserving context for logging and clients.
// Extension guidance: Add new variants or HTTP mappings here whenever a new subsystem introduces a failure mode.
// Security considerations: Avoid leaking sensitive details by returning sanitized messages in HTTP responses.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("required binary '{0}' missing from PATH")]
    MissingBinary(&'static str),
    #[error("inxi execution failed: {0}")]
    CommandFailure(String),
    #[error("invalid mode requested: {0}")]
    InvalidMode(String),
    #[error("asset not found: {0}")]
    AssetNotFound(String),
    #[error("failed to parse system report: {0}")]
    Parse(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::MissingBinary(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CommandFailure(_) => StatusCode::BAD_GATEWAY,
            AppError::InvalidMode(_) => StatusCode::BAD_REQUEST,
            AppError::AssetNotFound(_) => StatusCode::NOT_FOUND,
            AppError::Parse(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        tracing::warn!(error = %self, "handled request error");
        (
            status,
            Json(ErrorResponse {
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
