use axum::{Router, Server};
use std::net::SocketAddr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use quizmo::docs::ApiDoc;

#[tokio::main]
async fn main() {
    let api_doc = ApiDoc::openapi();
    
    let app_router = Router::new()
        // ...existing routes...
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://localhost:3000");
    println!("API documentation available at http://localhost:3000/swagger-ui/");

    Server::bind(&addr)
        .serve(app_router.into_make_service())
        .await
        .unwrap();
}
