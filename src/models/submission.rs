use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct QuizSubmission {
    pub id: String,
    pub quiz_id: String,
    pub user_id: String,
    pub answers: Vec<usize>,
    pub score: Option<i32>,
    pub submitted_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Answer {
    pub question_id: String,
    pub selected_option: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmitQuizDto {
    pub answers: Vec<Answer>,
}

impl QuizSubmission {
    pub fn new(quiz_id: String, user_id: String, answers: Vec<usize>, score: Option<i32>) -> Self {
        Self {
            id: nanoid::nanoid!(),
            quiz_id,
            user_id,
            answers,
            score,
            submitted_at: Utc::now().timestamp(),
        }
    }
}
