use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use quizmo::handlers;
use tower::ServiceExt;
use crate::common::TestContext;

#[tokio::test]
async fn test_create_quiz_handler() {
    let ctx = TestContext::new().await;
    let app = handlers::create_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"title":"Test Quiz","description":"Test Description"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_quiz_not_found() {
    let ctx = TestContext::new().await;
    let app = handlers::create_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/quizzes/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_update_quiz_handler() {
    let ctx = TestContext::new().await;
    let app = handlers::create_app().await;

    // First create a quiz
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"title":"Original Quiz","description":"Original Description"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // Then update it
    let update_response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/quizzes/1")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"title":"Updated Quiz","description":"Updated Description"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(update_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete_quiz_handler() {
    let ctx = TestContext::new().await;
    let app = handlers::create_app().await;

    // First create a quiz
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/quizzes")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"title":"Test Quiz","description":"Test Description"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::CREATED);

    // Then delete it
    let delete_response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/quizzes/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_list_quizzes_handler() {
    let ctx = TestContext::new().await;
    let app = handlers::create_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/quizzes")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
