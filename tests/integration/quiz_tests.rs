use anyhow::Result;
use quizmo::models::quiz::Quiz;
use quizmo::repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};
use tempfile::TempDir;

pub async fn test_quiz_crud_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = QuizRepositoryImpl::new_with_path(&temp_dir.path().to_path_buf())?;

    let quiz = Quiz {
        id: Some("test-quiz".to_string()),
        title: "Test Quiz".to_string(),
        description: Some("Test Description".to_string()),
        questions: vec![],
        created_at: None,
        updated_at: None,
    };

    // Test create
    let created = repo.create(quiz.clone()).await?;
    assert_eq!(created.title, quiz.title);

    // Test find by id
    let found = repo.find_by_id("test-quiz").await?;
    assert!(found.is_some());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_quiz_operations() -> Result<()> {
        test_quiz_crud_operations().await
    }
}
