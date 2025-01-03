use crate::common::TestContext;
use quizmo::models::test_types::TestQuiz;
use uuid::Uuid;

use quizmo::models::quiz::Quiz;
use quizmo::repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};

// Removed unused import
// use tempfile;

#[tokio::test]
async fn test_crud_operations() {
    let ctx = TestContext::new().await;
    let quiz = TestQuiz {
        id: Uuid::new_v4(), // Added `id`
        title: "CRUD Test Quiz".to_string(),
        description: "A quiz for testing CRUD operations".to_string(),
        questions: vec![], // Populate as needed
    };

    // Convert Uuid to String for repository operations
    let id = quiz.id.to_string();

    // Create
    let created = ctx.repo.create(Quiz::from(&quiz)).await.unwrap();
    assert_eq!(created.title, quiz.title);

    // Read
    let retrieved = ctx.repo.find_by_id(&id).await.unwrap().unwrap();
    assert_eq!(retrieved.title, quiz.title);

    // Update
    let mut updated_quiz = Quiz::from(&quiz);
    updated_quiz.title = "Updated Title".to_string();
    let updated = ctx
        .repo
        .update(&id, updated_quiz.clone())
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.title, "Updated Title");

    // Delete
    assert!(ctx.repo.delete(&id).await.unwrap());
    assert!(ctx.repo.find_by_id(&id).await.unwrap().is_none());
}

#[tokio::test]
async fn test_persistence() {
    let temp_dir = tempfile::tempdir().unwrap();
    let path = temp_dir.path().to_owned();

    // Create quiz in first instance
    {
        let repo = QuizRepositoryImpl::new_with_path(&path).unwrap();
        let quiz = TestQuiz {
            id: Uuid::new_v4(), // Added `id`
            title: "Persistent Quiz".to_string(),
            description: "A quiz that persists".to_string(),
            questions: vec![], // Populate as needed
        };
        repo.create(Quiz::from(&quiz)).await.unwrap();
    } // repo is dropped here

    // Open new repository instance and verify
    let new_repo = QuizRepositoryImpl::new_with_path(&path).unwrap();
    let quizzes = new_repo.find_all().await.unwrap(); // Use the correct method name
    assert_eq!(quizzes.len(), 1);
    assert_eq!(quizzes[0].title, "Persistent Quiz");
}
