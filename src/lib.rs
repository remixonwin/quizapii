pub mod models;
pub mod repository;
pub mod handlers;

#[cfg(test)]
mod tests {
    use crate::models::quiz::Quiz;
    use crate::repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};
    use anyhow::Result;
    use std::sync::Once;
    use chrono::Utc;
    use tempfile::TempDir;
    use std::path::Path;

    static INIT: Once = Once::new();

    async fn setup_test_db(dir: TempDir) -> Result<QuizRepositoryImpl> {
        Ok(QuizRepositoryImpl::new_with_path(dir.path())?)
    }

    #[tokio::test]
    async fn test_quiz_creation() {
        let dir = TempDir::new().unwrap();
        let repo = setup_test_db(dir).await.unwrap();
        let now = Utc::now();
        let quiz = Quiz {
            id: "test1".to_string(),
            title: "Test Quiz".to_string(),
            description: "Test quiz description".to_string(),
            questions: Vec::new(),
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
        };
        
        let result = repo.create(quiz.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().id, quiz.id);
    }

    #[tokio::test]
    async fn test_quiz_retrieval() {
        let dir = TempDir::new().unwrap();
        let repo = setup_test_db(dir).await.unwrap();
        let now = Utc::now();
        let quiz = Quiz {
            id: "test2".to_string(),
            title: "Test Quiz 2".to_string(),
            description: "Test quiz 2 description".to_string(),
            questions: Vec::new(),
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
        };
        
        let _ = repo.create(quiz.clone()).await;
        let result = repo.get(&quiz.id).await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, quiz.title);
    }

    #[tokio::test]
    async fn test_quiz_update() {
        let dir = TempDir::new().unwrap();
        let repo = setup_test_db(dir).await.unwrap();
        let now = Utc::now();
        let quiz = Quiz {
            id: "test3".to_string(),
            title: "Original Title".to_string(),
            description: "Original description".to_string(),
            questions: Vec::new(),
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
        };
        
        let _ = repo.create(quiz.clone()).await;
        
        let updated_quiz = Quiz {
            id: quiz.id.clone(),
            title: "Updated Title".to_string(),
            description: "Updated description".to_string(),
            questions: Vec::new(),
            created_at: quiz.created_at,
            updated_at: Utc::now().timestamp(),
        };
        
        let result = repo.update(&quiz.id, updated_quiz.clone()).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().title, "Updated Title");
    }

    #[tokio::test]
    async fn test_quiz_deletion() {
        let dir = TempDir::new().unwrap();
        let repo = setup_test_db(dir).await.unwrap();
        let now = Utc::now();
        let quiz = Quiz {
            id: "test4".to_string(),
            title: "To Be Deleted".to_string(),
            description: "Quiz to be deleted".to_string(),
            questions: Vec::new(),
            created_at: now.timestamp(),
            updated_at: now.timestamp(),
        };
        
        let _ = repo.create(quiz.clone()).await;
        let result = repo.delete(&quiz.id).await;
        assert!(result.is_ok());
        
        let get_result = repo.get(&quiz.id).await.unwrap();
        assert!(get_result.is_none());
    }
}
