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

// Re-export key types for easier access
pub use docs::ApiDoc;
pub use models::error::AppError;
pub use repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};

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
