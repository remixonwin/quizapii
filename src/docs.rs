use crate::models::{
    create_quiz_dto::CreateQuizDto, question::Question, quiz::Quiz, submission::*,
    update_quiz_dto::UpdateQuizDto,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        create_quiz,
        get_quiz,
        update_quiz,
        delete_quiz,
        list_quizzes,
        submit_quiz
    ),
    components(
        schemas(
            Quiz,
            Question,
            CreateQuizDto,
            UpdateQuizDto,
            QuizSubmission,
            SubmitQuizDto,
            Answer
        )
    ),
    tags(
        (name = "quizzes", description = "Quiz management endpoints"),
        (name = "submissions", description = "Quiz submission endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::HttpBuilder::new()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .build(),
                ),
            );
        }
    }
}

/// Create a new quiz
#[utoipa::path(
    post,
    path = "/api/quizzes",
    request_body = CreateQuizDto,
    responses(
        (status = 201, description = "Quiz created successfully", body = Quiz),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    tag = "quizzes"
)]
#[allow(dead_code)]
fn create_quiz() {}

/// Get a quiz by ID
#[utoipa::path(
    get,
    path = "/api/quizzes/{id}",
    responses(
        (status = 200, description = "Quiz found", body = Quiz),
        (status = 404, description = "Quiz not found")
    ),
    params(
        ("id" = String, Path, description = "Quiz ID")
    ),
    tag = "quizzes"
)]
#[allow(dead_code)]
fn get_quiz() {}

/// Update a quiz
#[utoipa::path(
    put,
    path = "/api/quizzes/{id}",
    request_body = UpdateQuizDto,
    responses(
        (status = 200, description = "Quiz updated successfully", body = Quiz),
        (status = 404, description = "Quiz not found"),
        (status = 400, description = "Invalid input")
    ),
    params(
        ("id" = String, Path, description = "Quiz ID")
    ),
    tag = "quizzes"
)]
#[allow(dead_code)]
fn update_quiz() {}

/// Delete a quiz
#[utoipa::path(
    delete,
    path = "/api/quizzes/{id}",
    responses(
        (status = 204, description = "Quiz deleted successfully"),
        (status = 404, description = "Quiz not found")
    ),
    params(
        ("id" = String, Path, description = "Quiz ID")
    ),
    tag = "quizzes"
)]
#[allow(dead_code)]
fn delete_quiz() {}

/// List all quizzes
#[utoipa::path(
    get,
    path = "/api/quizzes",
    responses(
        (status = 200, description = "List of quizzes", body = Vec<Quiz>)
    ),
    tag = "quizzes"
)]
#[allow(dead_code)]
fn list_quizzes() {}

/// Submit quiz answers
#[utoipa::path(
    post,
    path = "/api/quizzes/{id}/submit",
    request_body = SubmitQuizDto,
    responses(
        (status = 200, description = "Quiz submitted successfully", body = QuizSubmission),
        (status = 404, description = "Quiz not found"),
        (status = 400, description = "Invalid submission")
    ),
    params(
        ("id" = String, Path, description = "Quiz ID")
    ),
    tag = "submissions"
)]
#[allow(dead_code)]
fn submit_quiz() {}
