use async_trait::async_trait;
use entrait::Impl;

use crate::app_state::AppState;
use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;

#[async_trait]
impl AuditLogRepository for Impl<AppState> {
    async fn create_audit_log(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError> {
        let created = sqlx::query_as::<_, AuditLog>(include_str!("sql/audit_logs/create.sql"))
            .bind(audit_log.id)
            .bind(&audit_log.event_type)
            .bind(&audit_log.payload)
            .bind(audit_log.created_at)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(created)
    }

    async fn find_all_audit_logs(&self) -> Result<Vec<AuditLog>, AppError> {
        let logs = sqlx::query_as::<_, AuditLog>(include_str!("sql/audit_logs/find_all.sql"))
            .fetch_all(&self.db_pool)
            .await?;
        Ok(logs)
    }
}
