use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;
use quizapii::handlers;
use quizapii::models::quiz::Quiz;
use serde_json::json;

#[tokio::test]
async fn test_create_quiz_handler() {
    let app = handlers::create_app().await;
    let quiz = Quiz::new("Test".to_string(), vec![]);
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_string(&quiz).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_error_handling() {
    let app = handlers::create_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from("invalid json"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
