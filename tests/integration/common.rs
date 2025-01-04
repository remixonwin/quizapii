use axum::Router;
use quizmo::handlers;

pub async fn create_test_app() -> Router {
    handlers::create_app().await
}
