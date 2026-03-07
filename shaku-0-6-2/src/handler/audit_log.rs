use axum::Json;
use shaku_axum::Inject;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;
use crate::module::AppModule;

pub async fn list_audit_logs(
    repository: Inject<AppModule, dyn AuditLogRepository>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    let logs = repository.find_all().await?;
    Ok(Json(logs))
}
