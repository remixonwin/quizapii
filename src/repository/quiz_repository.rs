//! Quiz repository implementation
//! 
//! # Examples
//! 
//! ```
//! use quizmo::repository::quiz_repository::{QuizRepository, QuizRepositoryImpl};
//! use tempfile::TempDir;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let temp_dir = TempDir::new()?;
//!     let repo = QuizRepositoryImpl::new_with_path(&temp_dir.path().to_path_buf())?;
//!     
//!     // Test repository operations
//!     let quiz_id = "test_quiz";
//!     let result = repo.find_by_id(quiz_id).await?;
//!     assert!(result.is_none());
//!     Ok(())
//! }
//! ```

use crate::models::error::AppError;
use crate::models::quiz::Quiz;
use anyhow::Result;
use async_trait::async_trait;
use sled::Db;
use std::path::Path;

#[async_trait]
pub trait QuizRepository: Send + Sync {
    async fn create(&self, quiz: Quiz) -> Result<Quiz, AppError>;
    async fn find_all(&self) -> Result<Vec<Quiz>, AppError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Quiz>, AppError>;
    async fn update(&self, id: &str, quiz: Quiz) -> Result<Option<Quiz>, AppError>;
    async fn delete(&self, id: &str) -> Result<bool, AppError>;
}

pub struct QuizRepositoryImpl {
    db: Db,
}

impl QuizRepositoryImpl {
    pub fn new() -> Result<Self> {
        let db = sled::open("quiz_db")?;
        Ok(Self { db })
    }

    pub fn new_with_path(path: &Path) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub async fn get_all(&self) -> Result<Vec<Quiz>, AppError> {
        let mut quizzes = Vec::new();
        for item_result in self.db.iter() {
            let (_, value) = item_result.map_err(|e| AppError::DatabaseError(e.to_string()))?;
            let quiz: Quiz = serde_json::from_slice(&value)
                .map_err(|e| AppError::SerializationError(e.to_string()))?;
            quizzes.push(quiz);
        }
        Ok(quizzes)
    }
}

#[async_trait]
impl QuizRepository for QuizRepositoryImpl {
    async fn create(&self, quiz: Quiz) -> Result<Quiz, AppError> {
        let value =
            serde_json::to_vec(&quiz).map_err(|e| AppError::SerializationError(e.to_string()))?;

        let id = quiz
            .id
            .as_ref()
            .ok_or_else(|| AppError::InvalidInput("Quiz ID is required".to_string()))?;

        self.db
            .insert(id.as_bytes(), value)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.db
            .flush_async()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(quiz)
    }

    async fn find_all(&self) -> Result<Vec<Quiz>, AppError> {
        let mut quizzes = Vec::new();
        for item in self.db.iter() {
            let (_, value) = item?;
            let quiz: Quiz = serde_json::from_slice(&value)?;
            quizzes.push(quiz);
        }
        Ok(quizzes)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Quiz>, AppError> {
        if let Some(value) = self.db.get(id.as_bytes())? {
            let quiz: Quiz = serde_json::from_slice(&value)?;
            Ok(Some(quiz))
        } else {
            Ok(None)
        }
    }

    async fn update(&self, id: &str, quiz: Quiz) -> Result<Option<Quiz>, AppError> {
        if self.db.contains_key(id.as_bytes())? {
            let value = serde_json::to_vec(&quiz)?;
            self.db.insert(id.as_bytes(), value)?;
            self.db.flush_async().await?;
            Ok(Some(quiz))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &str) -> Result<bool, AppError> {
        if self.db.remove(id.as_bytes())?.is_some() {
            self.db.flush_async().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
