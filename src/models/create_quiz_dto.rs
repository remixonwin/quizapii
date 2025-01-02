use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateQuizDto {
    pub title: String,
    pub description: String,
    pub questions: Vec<QuestionDto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionDto {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub points: i32,
}
// Remove the conflicting From implementation
