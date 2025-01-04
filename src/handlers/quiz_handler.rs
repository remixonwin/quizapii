use std::sync::Arc; // Added import for Arc

use crate::{
    models::create_quiz_dto::CreateQuizDto, models::quiz::Quiz,
    models::update_quiz_dto::UpdateQuizDto, repository::quiz_repository::QuizRepository,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    response::{IntoResponse, Json as AxumJson},
};

pub async fn create_quiz(
    State(repo): State<Arc<dyn QuizRepository + Send + Sync>>, // Added `dyn`
    Json(dto): Json<CreateQuizDto>,
) -> Result<Json<Quiz>, StatusCode> {
    let quiz = Quiz::from(dto); // Ensure `From` trait is implemented
    repo.create(quiz)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_all_quizzes(
    State(repo): State<Arc<dyn QuizRepository + Send + Sync>>, // Added `dyn`
) -> Result<Json<Vec<Quiz>>, StatusCode> {
    repo.find_all()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_quiz(
    State(repo): State<Arc<dyn QuizRepository + Send + Sync>>, // Added `dyn`
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    match repo.find_by_id(&id).await {
        Ok(Some(quiz)) => Ok(AxumJson(quiz)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quiz(
    State(repo): State<Arc<dyn QuizRepository + Send + Sync>>, // Added `dyn`
    Path(id): Path<String>,
    Json(dto): Json<UpdateQuizDto>,
) -> Result<Json<Quiz>, StatusCode> {
    let updated_quiz: Quiz = dto.into(); // Convert UpdateQuizDto to Quiz
    match repo.update(&id, updated_quiz).await {
        Ok(Some(quiz)) => Ok(Json(quiz)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_quiz(
    State(repo): State<Arc<dyn QuizRepository + Send + Sync>>, // Added `dyn`
    Path(id): Path<String>,
) -> StatusCode {
    match repo.delete(&id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[cfg(test)]
mod tests {
    // Add tests for lines 14, 25, 34, 45, 58
}
