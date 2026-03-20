use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::error::AppError;
use crate::service::health_service_impl::HealthCheckService;

pub async fn health_check<S: HealthCheckService + Send + Sync + 'static>(
    State(app): State<Arc<S>>,
) -> Result<Json<Value>, AppError> {
    let status = app.check_health_service().await?;
    Ok(Json(json!({
        "status": status.overall(),
        "services": {
            "postgres": if status.postgres { "ok" } else { "error" },
            "redis": if status.redis { "ok" } else { "error" },
            "nats": if status.nats { "ok" } else { "error" },
        }
    })))
}
