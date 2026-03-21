use async_trait::async_trait;
use nject::injectable;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;
use crate::interface::audit_log_service::AuditLogService;

#[injectable]
pub(crate) struct AuditLogServiceImpl<'a> {
    repository: &'a dyn AuditLogRepository,
}

#[async_trait]
impl AuditLogService for AuditLogServiceImpl<'_> {
    async fn create(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError> {
        self.repository.create(audit_log).await
    }

    async fn find_all(&self) -> Result<Vec<AuditLog>, AppError> {
        self.repository.find_all().await
    }
}
