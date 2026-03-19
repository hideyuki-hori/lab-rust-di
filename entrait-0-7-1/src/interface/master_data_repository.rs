use async_trait::async_trait;
use entrait::entrait;

use crate::error::AppError;

#[entrait(mock_api = MasterDataRepositoryMock)]
#[async_trait]
pub trait MasterDataRepository {
    async fn get_master_data(&self, key: &str) -> Result<Option<String>, AppError>;
    async fn set_master_data(&self, key: &str, value: &str) -> Result<(), AppError>;
    async fn get_all_master_data(&self) -> Result<Vec<(String, String)>, AppError>;
}
