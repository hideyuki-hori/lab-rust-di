use async_trait::async_trait;
use entrait::entrait;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;

#[entrait]
#[async_trait]
pub trait AuditLogRepository {
    async fn create_audit_log(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError>;
    async fn find_all_audit_logs(&self) -> Result<Vec<AuditLog>, AppError>;
}
