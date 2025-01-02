use crate::common::{TestContext, create_test_user, setup};
use reqwest::StatusCode;

#[tokio::test]
async fn test_user_registration() {
    setup();
    let ctx = TestContext::new().await;
    let user = create_test_user();

    let response = ctx.register_user(&user).await;
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_user_login() {
    setup();
    let ctx = TestContext::new().await;
    let user = create_test_user();

    // First register the user
    ctx.register_user(&user).await;

    // Then try to login
    let response = ctx.login_user(&user).await;
    assert_eq!(response.status(), StatusCode::OK);

    // Verify we got a token back
    let json = response.json::<serde_json::Value>().await.unwrap();
    assert!(json["token"].is_string());
}

#[tokio::test]
async fn test_invalid_login() {
    setup();
    let ctx = TestContext::new().await;
    let mut user = create_test_user();
    
    // Register with correct password
    ctx.register_user(&user).await;
    
    // Try to login with wrong password
    user.password = "wrong_password".to_string();
    let response = ctx.login_user(&user).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}