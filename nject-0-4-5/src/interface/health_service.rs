use async_trait::async_trait;

use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait HealthService: Send + Sync {
    async fn check(&self) -> Result<HealthStatus, AppError>;
}

#[derive(Debug)]
pub struct HealthStatus {
    pub postgres: bool,
    pub redis: bool,
    pub nats: bool,
}

impl HealthStatus {
    pub fn overall(&self) -> &str {
        if self.postgres && self.redis && self.nats {
            "ok"
        } else {
            "degraded"
        }
    }
}
