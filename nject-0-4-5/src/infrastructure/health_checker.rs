use async_trait::async_trait;
use nject::inject;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use sqlx::PgPool;

use crate::error::AppError;
use crate::interface::health_service::{HealthService, HealthStatus};

#[inject(|pool: &'prov PgPool, redis_connection: &'prov ConnectionManager, nats_client: &'prov async_nats::Client| Self { pool: pool.clone(), redis_connection: redis_connection.clone(), nats_client: nats_client.clone() })]
pub(crate) struct HealthChecker {
    pool: PgPool,
    redis_connection: ConnectionManager,
    nats_client: async_nats::Client,
}

#[async_trait]
impl HealthService for HealthChecker {
    async fn check(&self) -> Result<HealthStatus, AppError> {
        let pg_ok = sqlx::query("SELECT 1").execute(&self.pool).await.is_ok();

        let redis_ok = {
            let mut conn = self.redis_connection.clone();
            conn.get::<_, Option<String>>("health:ping").await.is_ok()
        };

        let nats_ok = self.nats_client.flush().await.is_ok();

        Ok(HealthStatus {
            postgres: pg_ok,
            redis: redis_ok,
            nats: nats_ok,
        })
    }
}
