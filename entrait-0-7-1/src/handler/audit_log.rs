use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use entrait::Impl;

use crate::app_state::AppState;
use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;

pub async fn list_audit_logs(
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    let logs = app.find_all_audit_logs().await?;
    Ok(Json(logs))
}
