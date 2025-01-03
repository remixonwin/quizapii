use quizmo::{
    models::quiz::Quiz,
    repository::quiz_repository::QuizRepository,
};
use tempfile::tempdir;

#[tokio::test]
async fn test_quiz_repository_crud() {
    let temp_dir = tempdir().unwrap();
    let repo = QuizRepository::new(temp_dir.path()).await.unwrap();

    // Create
    let quiz = Quiz::new(
        "Test Quiz".to_string(),
        "Test Description".to_string(),
        vec![],
    );
    let created_id = repo.create(quiz).await.unwrap();

    // Read
    let retrieved = repo.get(created_id).await.unwrap();
    assert_eq!(retrieved.title, "Test Quiz");

    // Update
    let mut updated_quiz = retrieved;
    updated_quiz.update_title("Updated Quiz".to_string());
    repo.update(created_id, updated_quiz.clone()).await.unwrap();

    let retrieved_updated = repo.get(created_id).await.unwrap();
    assert_eq!(retrieved_updated.title, "Updated Quiz");

    // Delete
    repo.delete(created_id).await.unwrap();
    assert!(repo.get(created_id).await.is_err());
}

#[tokio::test]
async fn test_quiz_repository_list() {
    let temp_dir = tempdir().unwrap();
    let repo = QuizRepository::new(temp_dir.path()).await.unwrap();

    // Create multiple quizzes
    let quiz1 = Quiz::new(
        "Quiz 1".to_string(),
        "Description 1".to_string(),
        vec![],
    );
    let quiz2 = Quiz::new(
        "Quiz 2".to_string(),
        "Description 2".to_string(),
        vec![],
    );

    repo.create(quiz1).await.unwrap();
    repo.create(quiz2).await.unwrap();

    let all_quizzes = repo.list().await.unwrap();
    assert_eq!(all_quizzes.len(), 2);
}
