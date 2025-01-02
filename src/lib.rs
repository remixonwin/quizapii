use axum::{Router, Server};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod docs;
pub mod handlers;
pub mod models;
pub mod repository;

// Re-export key types for easier access
pub use docs::ApiDoc;
pub use models::error::AppError;
pub use repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};

pub async fn run_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    let addr = addr.parse()?;
    let app = Router::new()
        // Add your routes here
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}
