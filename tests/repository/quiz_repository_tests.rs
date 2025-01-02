use quizmo::repository::quiz_repository::QuizRepository;
use quizmo::models::quiz::Quiz;
use quizmo::models::question::Question;
use chrono::Utc;
use tempfile::TempDir;
use crate::common::{TestContext, create_test_quiz};

async fn setup_test_db() -> (QuizRepository, TempDir) {
    let temp_dir = tempfile::tempdir().unwrap();
    let repo = QuizRepository::new(temp_dir.path().to_str().unwrap()).await.unwrap();
    (repo, temp_dir)
}

#[tokio::test]
async fn test_create_and_get_quiz() {
    let ctx = TestContext::new().await;
    let quiz = create_test_quiz(None);

    let created = ctx.repo.create_quiz(&quiz).await.unwrap();
    assert_eq!(created.title, quiz.title);

    let retrieved = ctx.repo.get_quiz(&quiz.id).await.unwrap();
    assert_eq!(retrieved.id, quiz.id);
    assert_eq!(retrieved.title, quiz.title);
}

#[tokio::test]
async fn test_list_quizzes() {
    let ctx = TestContext::new().await;
    
    // Create multiple quizzes
    for i in 0..3 {
        let quiz = create_test_quiz(Some(format!("test{}", i)));
        ctx.repo.create_quiz(&quiz).await.unwrap();
    }

    let quizzes = ctx.repo.list_quizzes().await.unwrap();
    assert_eq!(quizzes.len(), 3);
}

#[tokio::test]
async fn test_update_quiz() {
    let ctx = TestContext::new().await;
    let mut quiz = create_test_quiz(None);

    ctx.repo.create_quiz(&quiz).await.unwrap();
    
    quiz.title = "Updated Title".to_string();
    let updated = ctx.repo.update_quiz(&quiz.id, &quiz).await.unwrap();
    assert_eq!(updated.title, "Updated Title");
}

#[tokio::test]
async fn test_delete_quiz() {
    let ctx = TestContext::new().await;
    let quiz = create_test_quiz(None);

    ctx.repo.create_quiz(&quiz).await.unwrap();
    ctx.repo.delete_quiz(&quiz.id).await.unwrap();

    let result = ctx.repo.get_quiz(&quiz.id).await;
    assert!(result.is_err());
}
