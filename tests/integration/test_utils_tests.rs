use crate::common::{create_test_app, setup_test_database};
use quizapii::test_utils::{create_test_quiz, setup_test_db, TestQuizRepository};

#[tokio::test]
async fn test_create_test_quiz() {
    let quiz = create_test_quiz();
    assert!(quiz.id.is_some());
    assert!(!quiz.title.is_empty());
    assert!(quiz.questions.is_some());
}

#[tokio::test]
async fn test_setup_test_db() {
    let db = setup_test_db().await;
    assert!(db.is_ok());
    
    let repo = TestQuizRepository::new();
    let quiz = create_test_quiz();
    let result = repo.create(quiz).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_test_repository() {
    let repo = TestQuizRepository::new();
    
    // Test create and find
    let quiz = create_test_quiz();
    let created = repo.create(quiz.clone()).await.unwrap();
    let found = repo.find_by_id(&created.id.unwrap()).await.unwrap();
    assert!(found.is_some());
    
    // Test find all
    let all = repo.find_all().await.unwrap();
    assert_eq!(all.len(), 1);
}

#[tokio::test]
async fn test_repository_crud_operations() {
    let repo = TestQuizRepository::new();
    let quiz = create_test_quiz();
    
    // Create
    let created = repo.create(quiz.clone()).await.unwrap();
    assert!(created.id.is_some());
    
    // Update
    let updated = repo.update(&created.id.unwrap(), Quiz::new("Updated".to_string(), vec![]))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.title, "Updated");
    
    // Delete
    let deleted = repo.delete(&created.id.unwrap()).await.unwrap();
    assert!(deleted);
    
    // Verify deletion
    let not_found = repo.find_by_id(&created.id.unwrap()).await.unwrap();
    assert!(not_found.is_none());
}

#[test]
fn test_repository_default() {
    let repo = TestQuizRepository::default();
    assert!(repo.quizzes.lock().unwrap().is_empty());
}

#[test]
fn run_test_create_quiz() {
    crate::run_test(async {
        let db = setup_test_database().await;
        let repo = TestQuizRepository::new();
        let quiz = create_test_quiz();
        let result = repo.create(quiz).await;
        assert!(result.is_ok());
    });
}
