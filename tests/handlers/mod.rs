use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use quizmo::handlers;
use tower::ServiceExt;
use crate::common::TestContext;
use serde_json::json;

pub mod quiz_handler_tests;
pub mod auth_handler_tests;

// Remove duplicate handler tests that exist in specific modules
