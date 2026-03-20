use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::service::master_data_service_impl::MasterDataService;

#[derive(Serialize)]
pub struct MasterDataEntry {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct SetMasterDataRequest {
    pub value: String,
}

pub async fn list_master_data<S: MasterDataService + Send + Sync + 'static>(
    State(app): State<Arc<S>>,
) -> Result<Json<Vec<MasterDataEntry>>, AppError> {
    let entries = app.get_all_master_data_service().await?;
    let result = entries
        .into_iter()
        .map(|(key, value)| MasterDataEntry { key, value })
        .collect();
    Ok(Json(result))
}

pub async fn get_master_data<S: MasterDataService + Send + Sync + 'static>(
    Path(key): Path<String>,
    State(app): State<Arc<S>>,
) -> Result<Json<MasterDataEntry>, AppError> {
    let value = app
        .get_master_data_service(&key)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Master data '{key}' not found")))?;
    Ok(Json(MasterDataEntry { key, value }))
}

pub async fn set_master_data<S: MasterDataService + Send + Sync + 'static>(
    Path(key): Path<String>,
    State(app): State<Arc<S>>,
    Json(req): Json<SetMasterDataRequest>,
) -> Result<StatusCode, AppError> {
    app.set_master_data_service(&key, &req.value).await?;
    Ok(StatusCode::NO_CONTENT)
}
