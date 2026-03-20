use entrait::entrait;

use crate::error::AppError;
use crate::interface::master_data_repository::MasterDataRepository;

#[entrait(pub MasterDataService)]
mod master_data_service {
    use super::*;

    pub async fn get_master_data_svc(
        deps: &impl MasterDataRepository,
        key: &str,
    ) -> Result<Option<String>, AppError> {
        deps.get_master_data(key).await
    }

    pub async fn set_master_data_svc(
        deps: &impl MasterDataRepository,
        key: &str,
        value: &str,
    ) -> Result<(), AppError> {
        deps.set_master_data(key, value).await
    }

    pub async fn get_all_master_data_svc(
        deps: &impl MasterDataRepository,
    ) -> Result<Vec<(String, String)>, AppError> {
        deps.get_all_master_data().await
    }
}
