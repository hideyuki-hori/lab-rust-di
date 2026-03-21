use async_trait::async_trait;
use nject::inject;
use sqlx::PgPool;

use crate::domain::audit_log::AuditLog;
use crate::error::AppError;
use crate::interface::audit_log_repository::AuditLogRepository;

#[inject(|pool: &'prov PgPool| Self { pool: pool.clone() })]
pub(crate) struct PostgresAuditLogRepository {
    pub(crate) pool: PgPool,
}

#[async_trait]
impl AuditLogRepository for PostgresAuditLogRepository {
    async fn create(&self, audit_log: &AuditLog) -> Result<AuditLog, AppError> {
        let created = sqlx::query_as::<_, AuditLog>(include_str!("sql/audit_logs/create.sql"))
            .bind(audit_log.id)
            .bind(&audit_log.event_type)
            .bind(&audit_log.payload)
            .bind(audit_log.created_at)
            .fetch_one(&self.pool)
            .await?;
        Ok(created)
    }

    async fn find_all(&self) -> Result<Vec<AuditLog>, AppError> {
        let logs = sqlx::query_as::<_, AuditLog>(include_str!("sql/audit_logs/find_all.sql"))
            .fetch_all(&self.pool)
            .await?;
        Ok(logs)
    }
}
