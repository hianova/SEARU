use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: bool,
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    AudioEncoding(String),
    NotFound(String),
    Conflict(String),
    Validation(String),
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::AudioEncoding(msg) => write!(f, "Audio encoding error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "RESOURCE_NOT_FOUND", msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT_ERROR", msg),
            AppError::AudioEncoding(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "AUDIO_ENCODING_ERROR", msg)
            }
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", msg),
        };

        let body = Json(ErrorResponse {
            error: true,
            code,
            message,
        });

        (status, body).into_response()
    }
}

impl From<hound::Error> for AppError {
    fn from(err: hound::Error) -> Self {
        AppError::AudioEncoding(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err = AppError::Validation("invalid range".to_string());
        assert_eq!(format!("{}", err), "Validation error: invalid range");

        let not_found = AppError::NotFound("file.wav".to_string());
        assert_eq!(format!("{}", not_found), "Not found: file.wav");

        let conflict = AppError::Conflict("already running".to_string());
        assert_eq!(format!("{}", conflict), "Conflict: already running");
    }
}

