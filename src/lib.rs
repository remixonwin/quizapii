//! Quizmo - A Quiz Management System
//! 
//! # Examples
//! 
//! ```
//! use quizmo::models::test_types::TestQuiz;
//! use uuid::Uuid;
//! 
//! let quiz = TestQuiz {
//!     id: Uuid::new_v4(),
//!     title: "Example Quiz".to_string(),
//!     description: "A simple example quiz".to_string(),
//!     questions: vec![],
//! };
//! assert_eq!(quiz.title, "Example Quiz");
//! ```
//! 
//! ## Error Handling
//! 
//! ```
//! use quizmo::AppError;
//! 
//! let error = AppError::NotFound;  // Changed: NotFound has no parameters
//! assert!(error.to_string().contains("Not found"));
//! ```

use axum::{Router, Server};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod test_utils;

// Re-export key types for easier access
pub use docs::ApiDoc;
pub use handlers::AppState;
pub use models::error::AppError;
pub use repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};
pub use test_utils::TestQuizRepository;

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;
    let app = Router::new()
        // Add your routes here
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

#[cfg(doctest)]
mod test {
    /// This function is used to ensure doctests are properly configured
    /// ```
    /// assert!(true);
    /// ```
    pub fn _dummy() {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde_json::json;
    use crate::models::{quiz::Quiz, question::Question, test_types::TestUser};

    #[tokio::test]
    async fn test_run_server_invalid_addr() {
        let result = run_server("invalid-address").await;
        assert!(result.is_err());
    }

    #[test]
    fn test_api_doc_generation() {
        let doc = ApiDoc::openapi();
        assert!(!doc.info.title.is_empty(), "API title should be set");
        assert!(!doc.info.version.is_empty(), "API version should be set");
    }

    #[tokio::test]
    async fn test_auth_operations() {
        let user = TestUser {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        
        // Test user registration
        let ctx = test_utils::TestContext::new().await;
        let app = ctx.app();
        
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/register")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&user).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);

        // Test invalid login
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/auth/login")
                    .header("Content-Type", "application/json")
                    .body(Body::from(json!({
                        "username": "wrong",
                        "password": "wrong"
                    }).to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_quiz_operations() {
        let ctx = test_utils::TestContext::new().await;
        let app = ctx.app();
        
        // Test quiz creation
        let quiz = Quiz::new(
            "Test Quiz".to_string(),
            "Test Description".to_string(),
            vec![],
        );
        
        let response = app
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/quizzes")
                    .header("Content-Type", "application/json")
                    .body(Body::from(serde_json::to_string(&quiz).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();
            
        assert_eq!(response.status(), StatusCode::CREATED);

        // Test quiz listing
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .method("GET")
                    .uri("/api/v1/quizzes")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
            
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_question_creation() {
        let question = Question::new(
            "Test Question".to_string(),
            vec!["Option 1".to_string(), "Option 2".to_string()],
            0,
            10,
        );

        assert_eq!(question.text, "Test Question");
        assert_eq!(question.options.len(), 2);
        assert_eq!(question.correct_option, 0);
        assert_eq!(question.points, 10);
        assert!(question.id.is_none());
    }

    mod error_tests {
        use super::*;

        #[test]
        fn test_error_responses() {
            let not_found = AppError::NotFound;
            let auth_error = AppError::AuthError;
            let validation_error = AppError::ValidationError("Invalid input".to_string());
            
            assert_eq!(
                not_found.into_response().status(),
                StatusCode::NOT_FOUND
            );
            assert_eq!(
                auth_error.into_response().status(),
                StatusCode::UNAUTHORIZED
            );
            assert_eq!(
                validation_error.into_response().status(),
                StatusCode::BAD_REQUEST
            );
        }
    }
}
