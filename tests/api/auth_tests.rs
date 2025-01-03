use crate::api::setup_test_context;
use crate::common::fixtures::create_test_user;
use axum::http::StatusCode;

#[tokio::test]
async fn test_user_registration() {
    let ctx = setup_test_context().await;
    let test_user = create_test_user();
    let response = ctx
        .register_user(&test_user)
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_user_login() {
    let ctx = setup_test_context().await;
    let test_user = create_test_user();

    // Register first
    ctx.register_user(&test_user)
        .await
        .expect("Failed to register");

    // Then login
    let response = ctx
        .login_user(&test_user)
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::OK);

    let json = response.json::<serde_json::Value>().await.unwrap();
    assert!(json.get("token").is_some());
}

#[tokio::test]
async fn test_invalid_login() {
    let ctx = setup_test_context().await;
    let mut test_user = create_test_user();
    test_user.password = "wrong_password".to_string();

    let response = ctx
        .login_user(&test_user)
        .await
        .expect("Failed to send request");
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
