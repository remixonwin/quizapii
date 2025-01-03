//! Error types for the application
//! 
//! # Examples
//! 
//! ```
//! use quizmo::AppError;
//! 
//! let not_found = AppError::NotFound;
//! assert!(not_found.to_string().contains("Not found"));
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    AuthError,
    #[error("Not found")]
    NotFound,
    #[error("Invalid input: {0}")]
    ValidationError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Internal server error")]
    InternalError,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
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
            AppError::AuthError => StatusCode::UNAUTHORIZED,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        };
        status.into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_not_found() {
        let err = AppError::NotFound;
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
        let err = AppError::InternalError;
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
}
