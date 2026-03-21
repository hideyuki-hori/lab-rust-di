use async_trait::async_trait;
use nject::injectable;

use crate::error::AppError;
use crate::infrastructure::health_checker::HealthChecker;
use crate::interface::health_service::{HealthService, HealthStatus};

#[injectable]
pub(crate) struct HealthServiceImpl<'a> {
    health_checker: &'a HealthChecker,
}

#[async_trait]
impl HealthService for HealthServiceImpl<'_> {
    async fn check(&self) -> Result<HealthStatus, AppError> {
        self.health_checker.check().await
    }
}
