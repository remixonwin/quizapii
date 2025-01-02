use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub created_at: i64,  // Unix timestamp
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuizDto {
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateQuizDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub questions: Option<Vec<Question>>,
}
