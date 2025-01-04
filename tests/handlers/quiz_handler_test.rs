use crate::common::test_helpers::setup_test_app;
use quizapii::models::quiz::Quiz;

#[tokio::test]
async fn test_create_quiz() {
    let app = setup_test_app().await;
    let quiz = Quiz::new("Test Quiz".to_string());
    
    let response = app
        .post("/api/quizzes")
        .json(&quiz)
        .send()
        .await;
        
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_get_quiz() {
    let app = setup_test_app().await;
    
    let response = app
        .get("/api/quizzes/1")
        .send()
        .await;
        
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_update_quiz() {
    let app = setup_test_app().await;
    let quiz = Quiz::new("Updated Quiz".to_string());
    
    let response = app
        .put("/api/quizzes/1")
        .json(&quiz)
        .send()
        .await;
        
    assert!(response.status().is_success());
}
