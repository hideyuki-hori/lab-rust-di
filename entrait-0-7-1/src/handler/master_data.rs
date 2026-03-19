use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use entrait::Impl;
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;
use crate::error::AppError;
use crate::interface::master_data_repository::MasterDataRepository;

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
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<Vec<MasterDataEntry>>, AppError> {
    let entries = app.get_all_master_data().await?;
    let result = entries
        .into_iter()
        .map(|(key, value)| MasterDataEntry { key, value })
        .collect();
    Ok(Json(result))
}

pub async fn get_master_data(
    Path(key): Path<String>,
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<MasterDataEntry>, AppError> {
    let value = app
        .get_master_data(&key)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Master data '{key}' not found")))?;
    Ok(Json(MasterDataEntry { key, value }))
}

pub async fn set_master_data(
    Path(key): Path<String>,
    State(app): State<Arc<Impl<AppState>>>,
    Json(req): Json<SetMasterDataRequest>,
) -> Result<StatusCode, AppError> {
    app.set_master_data(&key, &req.value).await?;
    Ok(StatusCode::NO_CONTENT)
}
