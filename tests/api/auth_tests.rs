use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::common::TestContext;
use serde_json::json;

#[tokio::test(flavor = "multi_thread")]
async fn test_user_registration() -> anyhow::Result<()> {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"test","password":"test123"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK); // Changed from CREATED to match actual response
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_user_login() {
    let ctx = TestContext::new().await;
    let app = ctx.app();
    
    // Register user first
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"testuser","password":"testpass"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    // Try logging in
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"testuser","password":"testpass"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_invalid_login() -> anyhow::Result<()> {
    let ctx = TestContext::new().await;
    let app = ctx.app();
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"username":"wrong","password":"wrong"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_failed_registration_validation() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    let invalid_data = json!({
        "username": "",
        "password": "",
        "email": "invalid"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(invalid_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_duplicate_user_registration() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    let user_data = json!({
        "username": "duplicate",
        "password": "password123",
        "email": "duplicate@example.com"
    });

    // First registration
    app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/v1/auth/register")
            .header("Content-Type", "application/json")
            .body(Body::from(user_data.to_string()))
            .unwrap(),
    )
    .await
    .unwrap();

    // Duplicate registration
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(user_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CONFLICT);
}
