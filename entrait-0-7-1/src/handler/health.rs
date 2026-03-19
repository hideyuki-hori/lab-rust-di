use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use entrait::Impl;
use serde_json::{json, Value};

use crate::app_state::AppState;
use crate::error::AppError;
use crate::interface::health_service::HealthService;

pub async fn health_check(State(app): State<Arc<Impl<AppState>>>) -> Result<Json<Value>, AppError> {
    let status = app.check_health().await?;
    Ok(Json(json!({
        "status": status.overall(),
        "services": {
            "postgres": if status.postgres { "ok" } else { "error" },
            "redis": if status.redis { "ok" } else { "error" },
            "nats": if status.nats { "ok" } else { "error" },
        }
    })))
}
