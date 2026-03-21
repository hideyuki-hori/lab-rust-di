use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use crate::error::AppError;
use crate::interface::health_service::HealthService;
use crate::provider::AppProvider;

pub async fn health_check(State(prov): State<Arc<AppProvider>>) -> Result<Json<Value>, AppError> {
    let service = prov.health_service();
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
