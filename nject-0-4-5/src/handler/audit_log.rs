use std::sync::Arc;

use axum::extract::State;
use axum::Json;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_service::AuditLogService;
use crate::provider::AppProvider;

pub async fn list_audit_logs(
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    let service = prov.audit_log_service();
    let logs = service.find_all().await?;
    Ok(Json(logs))
}
