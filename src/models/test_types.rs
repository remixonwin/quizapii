use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestQuestion {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestQuiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<TestQuestion>,
}
