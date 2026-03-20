use entrait::entrait;

use crate::error::AppError;
use crate::interface::health_service::{HealthService, HealthStatus};

#[entrait(pub HealthCheckService)]
mod health_check_service {
    use super::*;

    pub async fn check_health_svc(deps: &impl HealthService) -> Result<HealthStatus, AppError> {
        deps.check_health().await
    }
}
