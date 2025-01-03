//! Test types and utilities for testing
//! 
//! # Examples
//! 
//! ```
//! use quizmo::models::test_types::TestUser;
//! 
//! let user = TestUser {
//!     username: "testuser".to_string(),
//!     password: "password123".to_string(),
//! };
//! assert_eq!(user.username, "testuser");
//! ```

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestQuiz {
    pub id: Uuid, // Added `id` field
    pub title: String,
    pub description: String,
    pub questions: Vec<TestQuestion>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestQuestion {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestUser {
    pub username: String,
    pub password: String,
}
