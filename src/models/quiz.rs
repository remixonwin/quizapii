use crate::models::question::Question;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub created_at: i64,  // Unix timestamp
    pub updated_at: i64,
}

impl Quiz {
    pub fn new(title: String, description: String, questions: Vec<Question>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: nanoid::nanoid!(),
            title,
            description,
            questions,
            created_at: now,
            updated_at: now,
        }
    }
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
