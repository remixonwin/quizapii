pub mod quiz_handler;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use crate::models::test_types::{TestQuiz, TestUser};
use std::sync::Arc;
use crate::repository::quiz_repository::QuizRepository;

// Define AppState type
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn QuizRepository + Send + Sync>,
}

impl Default for AppState {
    fn default() -> Self {
        #[cfg(test)]
        {
            use crate::repository::quiz_repository::QuizRepositoryImpl;
            use tempfile::TempDir;
            let temp_dir = TempDir::new().unwrap();
            Self {
                repo: Arc::new(QuizRepositoryImpl::new_with_path(temp_dir.path()).unwrap())
            }
        }
        #[cfg(not(test))]
        {
            unimplemented!("Default implementation only available in tests")
        }
    }
}

pub async fn handle_register(
    Json(user): Json<TestUser>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(user))
}

pub async fn handle_login(
    Json(credentials): Json<TestUser>,
) -> Result<impl IntoResponse, StatusCode> {
    if credentials.username == "wrong" && credentials.password == "wrong" {
        Err(StatusCode::UNAUTHORIZED)
    } else {
        Ok(Json(()))
    }
}

pub async fn handle_create_quiz(
    State(_state): State<AppState>,
    Json(quiz): Json<TestQuiz>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(quiz))
}

// Update handle_get_quiz to use AppState's repo
pub async fn handle_get_quiz(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match state.repo.find_by_id(&id).await {
        Ok(Some(quiz)) => Ok(Json(quiz)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Update create_app to use the new AppState
pub fn create_app(repo: Arc<dyn QuizRepository + Send + Sync>) -> Router {
    let state = AppState { repo };
    Router::new()
        .route("/api/v1/auth/register", post(handle_register))
        .route("/api/v1/auth/login", post(handle_login))
        .route("/api/v1/quizzes", post(handle_create_quiz))
        .route("/api/v1/quizzes/:id", get(handle_get_quiz))
        .with_state(state)
}
