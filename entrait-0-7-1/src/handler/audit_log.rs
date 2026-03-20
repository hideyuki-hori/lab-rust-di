use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::service::audit_log_service_impl::AuditLogService;

pub async fn list_audit_logs<S: AuditLogService + Send + Sync + 'static>(
    State(app): State<Arc<S>>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    let logs = app.find_all_audit_logs_svc().await?;
    Ok(Json(logs))
}
