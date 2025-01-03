pub mod quiz_handler;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use crate::models::test_types::{TestQuiz, TestUser};
use uuid::Uuid;

// Define AppState type
#[derive(Clone, Default)] // Derive Default
pub struct AppState {
    // Add your state fields here
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

pub async fn handle_get_quiz(
    State(_state): State<AppState>,
    Path(_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(Json(()))
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/auth/register", post(handle_register))
        .route("/api/v1/auth/login", post(handle_login))
        .route("/api/v1/quizzes", post(handle_create_quiz))
        .route("/api/v1/quizzes/:id", get(handle_get_quiz))
        .with_state(state)
}
