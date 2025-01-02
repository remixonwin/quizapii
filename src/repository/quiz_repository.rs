use anyhow::Result;
use sled::Db;
use crate::models::quiz::Quiz;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait QuizRepository {
    async fn create(&self, quiz: Quiz) -> Result<Quiz>;
    async fn get(&self, id: &str) -> Result<Option<Quiz>>;
    async fn list(&self) -> Result<Vec<Quiz>>;
    async fn update(&self, id: &str, quiz: Quiz) -> Result<Option<Quiz>>;
    async fn delete(&self, id: &str) -> Result<bool>;
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
        Ok(QuizRepositoryImpl {
            db,
            // Initialize other fields as needed
        })
    }
}

#[async_trait]
impl QuizRepository for QuizRepositoryImpl {
    async fn create(&self, quiz: Quiz) -> Result<Quiz> {
        let value = serde_json::to_vec(&quiz)?;
        self.db.insert(quiz.id.as_bytes(), value)?;
        self.db.flush_async().await?;
        Ok(quiz)
    }

    async fn get(&self, id: &str) -> Result<Option<Quiz>> {
        if let Some(value) = self.db.get(id.as_bytes())? {
            let quiz: Quiz = serde_json::from_slice(&value)?;
            Ok(Some(quiz))
        } else {
            Ok(None)
        }
    }

    async fn list(&self) -> Result<Vec<Quiz>> {
        let mut quizzes = Vec::new();
        for item in self.db.iter() {
            let (_, value) = item?;
            let quiz: Quiz = serde_json::from_slice(&value)?;
            quizzes.push(quiz);
        }
        Ok(quizzes)
    }

    async fn update(&self, id: &str, quiz: Quiz) -> Result<Option<Quiz>> {
        if self.db.contains_key(id.as_bytes())? {
            let value = serde_json::to_vec(&quiz)?;
            self.db.insert(id.as_bytes(), value)?;
            self.db.flush_async().await?;
            Ok(Some(quiz))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        if self.db.remove(id.as_bytes())?.is_some() {
            self.db.flush_async().await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
