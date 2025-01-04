use crate::common::TestContext;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::json;

#[tokio::test]
async fn test_register_handler() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    let user_data = json!({
        "username": "newuser",
        "password": "password123",
        "email": "newuser@example.com"
    });

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

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_login_handler() {
    let ctx = TestContext::new().await;
    let app = ctx.app();

    // Register user first
    let user_data = json!({
        "username": "testlogin",
        "password": "password123",
        "email": "testlogin@example.com"
    });

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

    // Try logging in
    let login_data = json!({
        "username": "testlogin",
        "password": "password123"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(login_data.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
