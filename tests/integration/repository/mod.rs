use quizapii::repository::quiz_repository::QuizRepository;
use quizapii::models::quiz::Quiz;
use sqlx::PgPool;

#[tokio::test]
async fn test_repository_operations() {
    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    let repo = QuizRepository::new(pool);
    
    // Test create
    let quiz = Quiz::new("Test Quiz".to_string(), vec![]);
    let created = repo.create(quiz).await.unwrap();
    assert!(created.id.is_some());
    
    // Test find_all
    let all = repo.find_all().await.unwrap();
    assert!(!all.is_empty());
    
    // Test update
    let updated = repo.update(&created.id.unwrap(), Quiz::new("Updated".to_string(), vec![]))
        .await
        .unwrap();
    assert!(updated.is_some());
    
    // Test delete
    let deleted = repo.delete(&created.id.unwrap()).await.unwrap();
    assert!(deleted);
}
