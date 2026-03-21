use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::interface::master_data_service::MasterDataService;
use crate::provider::AppProvider;

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
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<Vec<MasterDataEntry>>, AppError> {
    let service = prov.master_data_service();
    let entries = service.get_all().await?;
    let result = entries
        .into_iter()
        .map(|(key, value)| MasterDataEntry { key, value })
        .collect();
    Ok(Json(result))
}

pub async fn get_master_data(
    Path(key): Path<String>,
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<MasterDataEntry>, AppError> {
    let service = prov.master_data_service();
    let value = service
        .get(&key)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Master data '{key}' not found")))?;
    Ok(Json(MasterDataEntry { key, value }))
}

pub async fn set_master_data(
    Path(key): Path<String>,
    State(prov): State<Arc<AppProvider>>,
    Json(req): Json<SetMasterDataRequest>,
) -> Result<StatusCode, AppError> {
    if req.value.is_empty() {
        return Err(AppError::BadRequest(
            "Master data value must not be empty".to_string(),
        ));
    }
    let service = prov.master_data_service();
    service.set(&key, &req.value).await?;
    Ok(StatusCode::NO_CONTENT)
}
