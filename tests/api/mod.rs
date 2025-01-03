mod auth_tests;
pub mod quiz_tests;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::common::server::setup_test_server;
use crate::common::TestContext;

#[allow(dead_code)]
pub async fn setup() -> String {
    setup_test_server()
        .await
        .expect("Failed to start test server")
}

pub async fn setup_test_context() -> TestContext {
    TestContext::new().await
}

#[tokio::test]
async fn test_invalid_login() {
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
}
