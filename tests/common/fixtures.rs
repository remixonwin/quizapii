use quizmo::models::test_types::TestUser;

/// Creates a test user for authentication tests
#[allow(dead_code)]
pub fn create_test_user() -> TestUser {
    TestUser {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    }
}
