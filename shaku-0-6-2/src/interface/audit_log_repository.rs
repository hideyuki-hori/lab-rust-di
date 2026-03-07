use async_trait::async_trait;
use shaku::Interface;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait AuditLogRepository: Interface {
    async fn create(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError>;
    async fn find_all(&self) -> Result<Vec<AuditLog>, AppError>;
}
