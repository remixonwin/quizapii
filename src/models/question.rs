use crate::models::create_quiz_dto::QuestionDto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Represents a quiz question
///
/// # Examples
///
/// ```
/// use quizmo::models::question::Question;
///
/// let question = Question {
///     id: Some("123".to_string()),
///     text: "What is Rust?".to_string(),
///     options: vec!["A programming language".to_string(), "A metal".to_string()],
///     correct_option: 0,
///     points: 10,
/// };
///
/// assert_eq!(question.text, "What is Rust?");
/// assert_eq!(question.correct_option, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct Question {
    pub id: Option<String>,
    pub text: String,
    pub options: Vec<String>,
    pub correct_option: usize,
    pub points: i32,
}

impl Question {
    pub fn new(text: String, options: Vec<String>, correct_option: usize, points: i32) -> Self {
        Self {
            id: None,
            text,
            options,
            correct_option,
            points,
        }
    }
}

impl From<QuestionDto> for Question {
    fn from(dto: QuestionDto) -> Self {
        Question::new(
            dto.text,
            dto.options,
            dto.correct_answer, // Changed from correct_option to correct_answer
            dto.points,
        )
    }
}
