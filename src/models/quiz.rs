use crate::models::create_quiz_dto::CreateQuizDto;
use crate::models::question::Question;
use crate::models::test_types::TestQuiz; // Add this import
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Quiz {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
    pub created_at: i64, // Unix timestamp
    pub updated_at: i64,
}

impl Quiz {
    pub fn new(title: String, description: String, questions: Vec<Question>) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: Some(nanoid::nanoid!()),
            title,
            description,
            questions,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<CreateQuizDto> for Quiz {
    fn from(dto: CreateQuizDto) -> Self {
        Quiz {
            id: Some(Uuid::new_v4().to_string()), // Convert Uuid to String
            title: dto.title,
            description: dto.description,
            questions: dto.questions.into_iter().map(Question::from).collect(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
}

impl From<&TestQuiz> for Quiz {
    fn from(test_quiz: &TestQuiz) -> Self {
        Quiz {
            id: Some(test_quiz.id.to_string()), // Convert Uuid to String
            title: test_quiz.title.clone(),
            description: test_quiz.description.clone(),
            questions: test_quiz
                .questions
                .iter()
                .map(|q| Question::new(q.text.clone(), q.options.clone(), q.correct_answer, 10))
                .collect(),
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
}
