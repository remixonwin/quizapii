use crate::common::TestContext;
use quizmo::models::user::User;
use uuid::Uuid;

#[tokio::test]
async fn test_user_crud_operations() {
    let ctx = TestContext::new().await;
    
    let user = User {
        id: Uuid::new_v4(),
        username: "testuser".to_string(),
        password_hash: "hashedpassword".to_string(),
        email: "test@example.com".to_string(),
        created_at: chrono::Utc::now(),
    };

    // Create
    let created = ctx.user_repo.create(&user).await.unwrap();
    assert_eq!(created.username, user.username);

    // Read
    let retrieved = ctx.user_repo.find_by_id(&user.id.to_string()).await.unwrap().unwrap();
    assert_eq!(retrieved.username, user.username);

    // Update
    let mut updated_user = user.clone();
    updated_user.username = "updated_user".to_string();
    let updated = ctx.user_repo.update(&user.id.to_string(), &updated_user).await.unwrap();
    assert_eq!(updated.username, "updated_user");

    // Delete
    ctx.user_repo.delete(&user.id.to_string()).await.unwrap();
    assert!(ctx.user_repo.find_by_id(&user.id.to_string()).await.unwrap().is_none());
}

#[tokio::test]
async fn test_find_by_username() {
    let ctx = TestContext::new().await;
    
    let user = User {
        id: Uuid::new_v4(),
        username: "findme".to_string(),
        password_hash: "hashedpassword".to_string(),
        email: "findme@example.com".to_string(),
        created_at: chrono::Utc::now(),
    };

    ctx.user_repo.create(&user).await.unwrap();

    let found = ctx.user_repo.find_by_username("findme").await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().email, user.email);
}
