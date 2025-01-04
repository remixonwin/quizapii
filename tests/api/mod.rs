pub mod auth_tests;
pub mod quiz_tests;

use crate::common::{TestContext, assertions::*};
use axum::{body::Body, http::{Request, StatusCode}};

async fn make_test_request(
    ctx: &TestContext,
    method: &str,
    uri: &str,
    body: Option<String>,
) -> StatusCode {
    let app = ctx.app();
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("Content-Type", "application/json");

    let body = match body {
        Some(b) => Body::from(b),
        None => Body::empty(),
    };

    let response = app
        .oneshot(builder.body(body).unwrap())
        .await
        .unwrap();

    response.status()
}
