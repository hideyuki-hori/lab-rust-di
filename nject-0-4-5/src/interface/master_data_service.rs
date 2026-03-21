use async_trait::async_trait;

use crate::error::AppError;

#[async_trait]
pub trait MasterDataService: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, AppError>;
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError>;
    async fn get_all(&self) -> Result<Vec<(String, String)>, AppError>;
}
