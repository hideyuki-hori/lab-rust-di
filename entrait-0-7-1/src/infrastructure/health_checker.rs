use async_trait::async_trait;
use entrait::Impl;
use redis::AsyncCommands;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::interface::health_service::{HealthService, HealthStatus};

#[async_trait]
impl HealthService for Impl<AppState> {
    async fn check_health(&self) -> Result<HealthStatus, AppError> {
        let pg_ok = sqlx::query("SELECT 1").execute(&self.db_pool).await.is_ok();

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
