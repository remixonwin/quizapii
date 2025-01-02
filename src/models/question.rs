use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Question {
    pub id: String,
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub points: u32,
}

impl Question {
    pub fn new(text: String, options: Vec<String>, correct_answer: usize, points: u32) -> Self {
        Self {
            id: nanoid::nanoid!(),
            text,
            options,
            correct_answer,
            points,
        }
    }
}
