use async_trait::async_trait;
use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use shaku::Component;
use sqlx::PgPool;

use crate::error::AppError;
use crate::interface::health_service::{HealthService, HealthStatus};

#[derive(Component)]
#[shaku(interface = HealthService)]
pub struct HealthChecker {
    pool: PgPool,
    redis_conn: ConnectionManager,
    nats_client: async_nats::Client,
}

#[async_trait]
impl HealthService for HealthChecker {
    async fn check(&self) -> Result<HealthStatus, AppError> {
        let pg_ok = sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .is_ok();

        let redis_ok = {
            let mut conn = self.redis_conn.clone();
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
