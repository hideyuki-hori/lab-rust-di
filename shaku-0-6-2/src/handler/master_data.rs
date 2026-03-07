use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use shaku_axum::Inject;

use crate::error::AppError;
use crate::interface::master_data_repository::MasterDataRepository;
use crate::module::AppModule;

#[derive(Serialize)]
pub struct MasterDataEntry {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SetMasterDataRequest {
    pub value: String,
}

pub async fn list_master_data(
    repository: Inject<AppModule, dyn MasterDataRepository>,
) -> Result<Json<Vec<MasterDataEntry>>, AppError> {
    let entries = repository.get_all().await?;
    let result = entries
        .into_iter()
        .map(|(key, value)| MasterDataEntry { key, value })
        .collect();
    Ok(Json(result))
}

pub async fn get_master_data(
    Path(key): Path<String>,
    repository: Inject<AppModule, dyn MasterDataRepository>,
) -> Result<Json<MasterDataEntry>, AppError> {
    let value = repository
        .get(&key)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Master data '{key}' not found")))?;
    Ok(Json(MasterDataEntry { key, value }))
}

pub async fn set_master_data(
    Path(key): Path<String>,
    repository: Inject<AppModule, dyn MasterDataRepository>,
    Json(req): Json<SetMasterDataRequest>,
) -> Result<StatusCode, AppError> {
    repository.set(&key, &req.value).await?;
    Ok(StatusCode::NO_CONTENT)
}
