pub mod quiz_tests;

use crate::common::TestContext;
use quizmo::repository::Repository;

pub async fn test_repository_operations<T: Repository>(repo: T) {
    // Generic repository test implementation
    // ...existing code...
}

use uuid::Uuid;
use tempfile::TempDir;
use std::path::PathBuf;
