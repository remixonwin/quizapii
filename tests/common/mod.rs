pub mod context;
pub mod fixtures;
pub mod server;

pub use context::TestContext;
pub use quizmo::test_utils::TestQuizRepository;

use quizmo::models::quiz::Quiz;

pub fn create_test_quiz(title: Option<String>) -> Quiz {
    Quiz {
        id: Some(nanoid::nanoid!()),
        title: title.unwrap_or_else(|| "Test Quiz".to_string()),
        description: "Test Description".to_string(),
        questions: vec![],
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    }
}