use async_trait::async_trait;
use nject::injectable;

use crate::error::AppError;
use crate::interface::master_data_repository::MasterDataRepository;
use crate::interface::master_data_service::MasterDataService;

#[injectable]
pub(crate) struct MasterDataServiceImpl<'a> {
    repository: &'a dyn MasterDataRepository,
}

#[async_trait]
impl MasterDataService for MasterDataServiceImpl<'_> {
    async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        self.repository.get(key).await
    }

    async fn set(&self, key: &str, value: &str) -> Result<(), AppError> {
        self.repository.set(key, value).await
    }

    async fn get_all(&self) -> Result<Vec<(String, String)>, AppError> {
        self.repository.get_all().await
    }
}
