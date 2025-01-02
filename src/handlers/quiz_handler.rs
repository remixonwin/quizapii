use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::{
    models::quiz::{Quiz, CreateQuizDto, UpdateQuizDto},
    repository::quiz_repository::QuizRepository,
};

pub async fn create_quiz(
    State(repo): State<QuizRepository>,
    Json(dto): Json<CreateQuizDto>,
) -> Result<Json<Quiz>, StatusCode> {
    repo.create(dto)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_all_quizzes(
    State(repo): State<QuizRepository>,
) -> Result<Json<Vec<Quiz>>, StatusCode> {
    repo.find_all()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_quiz(
    State(repo): State<QuizRepository>,
    Path(id): Path<String>,
) -> Result<Json<Quiz>, StatusCode> {
    match repo.find_by_id(&id).await {
        Ok(Some(quiz)) => Ok(Json(quiz)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quiz(
    State(repo): State<QuizRepository>,
    Path(id): Path<String>,
    Json(dto): Json<UpdateQuizDto>,
) -> Result<Json<Quiz>, StatusCode> {
    match repo.update(&id, dto).await {
        Ok(Some(quiz)) => Ok(Json(quiz)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_quiz(
    State(repo): State<QuizRepository>,
    Path(id): Path<String>,
) -> StatusCode {
    match repo.delete(&id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
