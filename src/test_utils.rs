use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::models::quiz::Quiz;
use crate::models::error::AppError;
use crate::repository::quiz_repository::QuizRepository;

#[derive(Clone)]
pub struct TestQuizRepository {
    quizzes: Arc<Mutex<HashMap<String, Quiz>>>,
}

impl TestQuizRepository {
    pub fn new() -> Self {
        Self {
            quizzes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for TestQuizRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl QuizRepository for TestQuizRepository {
    async fn find_by_id(&self, id: &str) -> Result<Option<Quiz>, AppError> {
        Ok(self.quizzes.lock().map_err(|_| AppError::InternalError("Lock error".to_string()))?.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Quiz>, AppError> {
        Ok(self.quizzes.lock().unwrap().values().cloned().collect())
    }

    async fn create(&self, quiz: Quiz) -> Result<Quiz, AppError> {
        let mut quizzes = self.quizzes.lock().unwrap();
        let id = quiz.id.clone().unwrap_or_else(|| nanoid::nanoid!());
        let quiz = Quiz { id: Some(id.clone()), ..quiz };
        quizzes.insert(id, quiz.clone());
        Ok(quiz)
    }

    async fn update(&self, id: &str, quiz: Quiz) -> Result<Option<Quiz>, AppError> {
        let mut quizzes = self.quizzes.lock().unwrap();
        if quizzes.contains_key(id) {
            let quiz = Quiz { id: Some(id.to_string()), ..quiz };
            quizzes.insert(id.to_string(), quiz.clone());
            Ok(Some(quiz))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &str) -> Result<bool, AppError> {
        let mut quizzes = self.quizzes.lock().unwrap();
        Ok(quizzes.remove(id).is_some())
    }
}

pub async fn concurrently_update_quiz(
    repo: &TestQuizRepository,
    quiz_id: &str,
) -> Result<(), AppError> {
    let mut handles = vec![];
    for i in 0..5 {
        let repo_clone = repo.clone();
        let id_clone = quiz_id.to_string();
        handles.push(tokio::spawn(async move {
            let quiz_update = Quiz {
                id: Some(id_clone),
                title: format!("Concurrent Update {}", i),
                description: "Test description".to_string(),
                questions: Vec::new(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            repo_clone.update(&id_clone, quiz_update).await
        }));
    }
    for handle in handles {
        handle.await??;
    }
    Ok(())
}
