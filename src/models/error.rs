//! Error types for the application
//! 
//! # Examples
//! 
//! ```
//! use quizmo::AppError;
//! 
//! let not_found = AppError::NotFound("Resource not found".to_string());
//! assert!(not_found.to_string().contains("Resource not found"));
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Validation error: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Internal server error: {0}")]
    InternalError(String),
    #[error("Authorization error: {0}")]
    AuthError(String),
    #[error("Concurrent modification error: {0}")]
    ConcurrencyError(String),
}

// Implement conversion from sled::Error
impl From<sled::Error> for AppError {
    fn from(err: sled::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

// Implement conversion from serde_json::Error
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ConcurrencyError(_) => StatusCode::CONFLICT,
        };
        status.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::NotFound("Resource not found".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_bad_request() {
        let err = AppError::ValidationError("Invalid input".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_app_error_internal() {
        let err = AppError::InternalError("Internal server error".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_database() {
        let err = AppError::DatabaseError("DB connection failed".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_serialization() {
        let err = AppError::SerializationError("JSON decode failed".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_concurrency() {
        let err = AppError::ConcurrencyError("Concurrent modification error".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }
}
