use crate::models::{
    create_quiz_dto::CreateQuizDto,
    question::Question,
    test_types::TestQuiz,
    validation::{Validatable, validate_string_not_empty, validate_vec_not_empty},
    error::AppError
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use uuid::Uuid;
use nanoid;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Quiz {
    #[schema(example = "123e4567-e89b-12d3-a456-426614174000")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[validate(length(min = 1, max = 100))]
    pub title: String,

    #[validate(length(min = 1))]
    pub description: String,

    #[validate]
    pub questions: Vec<Question>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,

    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>
}

impl Validatable for Quiz {
    fn validate(&self) -> Result<(), AppError> {
        validate_string_not_empty("title", &self.title)?;
        validate_string_not_empty("description", &self.description)?;
        validate_vec_not_empty("questions", &self.questions)?;
        
        for (i, question) in self.questions.iter().enumerate() {
            question.validate().map_err(|e| 
                AppError::ValidationError(format!("Question {}: {}", i, e))
            )?;
        }
        Ok(())
    }
}

impl Quiz {
    pub fn new(title: String, description: String, questions: Vec<Question>) -> Self {
        let now = Utc::now();
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
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
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Add tests for lines 21-22, 24, 35, 37-42
}
