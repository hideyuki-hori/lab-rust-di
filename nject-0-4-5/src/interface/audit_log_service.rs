use async_trait::async_trait;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;

#[async_trait]
pub trait AuditLogService: Send + Sync {
    async fn create(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError>;
    async fn find_all(&self) -> Result<Vec<AuditLog>, AppError>;
}
