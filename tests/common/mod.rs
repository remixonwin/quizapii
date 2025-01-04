pub mod context;
pub mod fixtures;
pub mod server;
pub mod test_utils;

pub use context::TestContext;
pub use quizmo::test_utils::TestQuizRepository;

use quizmo::models::{quiz::Quiz, user::User, question::Question};
use quizmo::repository::{quiz_repository::QuizRepository, user_repository::UserRepository};
use std::sync::Once;
use tempfile::TempDir;
use tokio::runtime::Runtime;
use axum::Router;
use uuid::Uuid;
use serde_json::Value;
use axum::{body::Body, http::{Request, StatusCode}};

static INIT: Once = Once::new();

pub fn initialize_tests() {
    INIT.call_once(|| {
        env_logger::init();
    });
}

pub fn test_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

pub async fn setup_test_database() -> sqlx::PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/quizmo_test".to_string());
    
    sqlx::PgPool::connect(&database_url).await.unwrap()
}

pub async fn setup_test_app() -> Router {
    // Common test app setup
    // ...existing code...
}

pub async fn create_test_quiz(title: Option<String>) -> Quiz {
    Quiz::new(
        title.unwrap_or_else(|| format!("Test Quiz {}", Uuid::new_v4())),
        "Test Description".to_string(),
        vec![create_test_question()],
    )
}

pub fn create_test_question() -> Question {
    Question::new(
        "Test Question".to_string(),
        vec!["Option A".to_string(), "Option B".to_string()],
        0,
        10,
    )
}

pub struct TestContext {
    pub app: Router,
    pub repo: Arc<dyn QuizRepository>,
}

impl TestContext {
    pub async fn new() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        let quiz_repo = QuizRepository::new(temp_dir.path()).await.unwrap();
        let user_repo = UserRepository::new(temp_dir.path()).await.unwrap();
        let app = setup_test_app().await;
        
        Self {
            quiz_repo,
            user_repo,
            app,
            _temp_dir: temp_dir,
        }
    }
}

pub async fn make_test_request(
    ctx: &TestContext,
    method: &str,
    uri: &str,
    body: Option<String>,
) -> StatusCode {
    // Move from api/mod.rs
    // ...existing code...
}

pub mod assertions {
    use axum::http::StatusCode;
    use serde_json::Value;

    pub fn assert_success_response(status: StatusCode) {
        assert!(status.is_success(), "Expected success status, got {}", status);
    }

    pub fn assert_error_response(status: StatusCode, expected: StatusCode) {
        assert_eq!(status, expected, "Expected status {}, got {}", expected, status);
    }

    pub fn assert_valid_quiz_response(json: Value) {
        assert!(json.get("id").is_some(), "Missing quiz id");
        assert!(json.get("title").is_some(), "Missing quiz title");
        // ...more assertions...
    }
}

pub mod test_utils {
    // Move test utilities from multiple files here
    // ...existing code...
}