use super::common::{setup_test_quiz, create_multiple_quizzes};
use crate::test_utils::TestQuizRepository;

#[tokio::test]
async fn test_quiz_crud_operations() {
    let (repo, quiz_id) = setup_test_quiz().await;
    
    // Test find
    let found = repo.find_by_id(&quiz_id).await.unwrap().unwrap();
    assert_eq!(found.id.unwrap(), quiz_id);
    
    // Test update
    let mut quiz_update = found.clone();
    quiz_update.title = "Updated Title".to_string();
    let updated = repo.update(&quiz_id, quiz_update).await.unwrap().unwrap();
    assert_eq!(updated.title, "Updated Title");
    
    // Test delete
    assert!(repo.delete(&quiz_id).await.unwrap());
}

#[tokio::test]
async fn test_find_all_quizzes() {
    let repo = TestQuizRepository::new();
    let ids = create_multiple_quizzes(&repo, 5).await;
    
    let all_quizzes = repo.find_all().await.unwrap();
    assert_eq!(all_quizzes.len(), 5);
    
    for quiz in all_quizzes {
        assert!(ids.contains(&quiz.id.unwrap()));
    }
}
