use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use crate::models::error::AppError;

#[async_trait]
pub trait Repository<T>
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, AppError>;
    async fn find_all(&self) -> Result<Vec<T>, AppError>;
    async fn create(&self, item: T) -> Result<T, AppError>;
    async fn update(&self, id: &str, item: T) -> Result<Option<T>, AppError>;
    async fn delete(&self, id: &str) -> Result<bool, AppError>;
    
    async fn upsert(&self, id: &str, item: T) -> Result<T, AppError> {
        match self.update(id, item.clone()).await? {
            Some(updated) => Ok(updated),
            None => self.create(item).await,
        }
    }
}
