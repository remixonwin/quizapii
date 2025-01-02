use crate::models::question::Question;
use crate::models::quiz::Quiz;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UpdateQuizDto {
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

impl From<UpdateQuizDto> for Quiz {
    fn from(dto: UpdateQuizDto) -> Self {
        Quiz {
            id: None, // ID should not be updated here
            title: dto.title,
            description: dto.description,
            questions: dto.questions.into_iter().map(Question::from).collect(),
            created_at: Utc::now().timestamp(), // This should probably remain unchanged; adjust as necessary
            updated_at: Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuestionDto {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub points: i32,
}

impl From<QuestionDto> for Question {
    fn from(dto: QuestionDto) -> Self {
        Question::new(dto.text, dto.options, dto.correct_answer, dto.points)
    }
}
