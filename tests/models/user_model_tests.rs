use quizmo::models::user::User;

pub fn test_user_model_operations() {
    let user = User {
        id: Some("test-id".to_string()),
        username: "test_user".to_string(),
        password_hash: "hashed_password".to_string(),
    };

    assert_eq!(user.username, "test_user");
    assert!(!user.password_hash.is_empty());
}
