use async_trait::async_trait;
use entrait::entrait;

use crate::error::AppError;

#[entrait]
#[async_trait]
pub trait HealthService {
    async fn check_health(&self) -> Result<HealthStatus, AppError>;
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
