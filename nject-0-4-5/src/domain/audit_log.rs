use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::value_objects::AuditLogId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "snake_case")]
pub enum EventType {
    #[serde(rename = "order.created")]
    #[sqlx(rename = "order.created")]
    OrderCreated,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OrderCreated => write!(f, "order.created"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: AuditLogId,
    pub event_type: EventType,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
