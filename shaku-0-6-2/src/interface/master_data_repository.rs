use async_trait::async_trait;
use shaku::Interface;

use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MasterDataRepository: Interface {
    async fn get(&self, key: &str) -> Result<Option<String>, AppError>;
    async fn set(&self, key: &str, value: &str) -> Result<(), AppError>;
    async fn get_all(&self) -> Result<Vec<(String, String)>, AppError>;
}
