use entrait::entrait;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;

#[entrait(pub AuditLogService)]
mod audit_log_service {
    use super::*;

    pub async fn create_audit_log_service(
        deps: &impl AuditLogRepository,
        audit_log: &AuditLog,
    ) -> Result<AuditLog, AppError> {
        deps.create_audit_log(audit_log).await
    }

    pub async fn find_all_audit_logs_service(
        deps: &impl AuditLogRepository,
    ) -> Result<Vec<AuditLog>, AppError> {
        deps.find_all_audit_logs().await
    }
}
