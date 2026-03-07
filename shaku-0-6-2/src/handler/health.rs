use axum::Json;
use serde_json::{json, Value};
use shaku_axum::Inject;

use crate::error::AppError;
use crate::interface::health_service::HealthService;
use crate::module::AppModule;

pub async fn health_check(
    service: Inject<AppModule, dyn HealthService>,
) -> Result<Json<Value>, AppError> {
    let status = service.check().await?;
    Ok(Json(json!({
        "status": status.overall(),
        "services": {
            "postgres": if status.postgres { "ok" } else { "error" },
            "redis": if status.redis { "ok" } else { "error" },
            "nats": if status.nats { "ok" } else { "error" },
        }
    })))
}
