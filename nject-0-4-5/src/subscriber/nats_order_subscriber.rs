use std::sync::Arc;

use chrono::Utc;
use tokio_stream::StreamExt;

use crate::domain::audit_log::{AuditLog, EventType};
use crate::domain::order::OrderEvent;
use crate::domain::value_objects::AuditLogId;
use crate::interface::audit_log_service::AuditLogService;
use crate::provider::AppProvider;

pub fn spawn(nats_client: async_nats::Client, provider: Arc<AppProvider>) {
    tokio::spawn(async move {
        if let Err(e) = subscribe(nats_client, &provider).await {
            tracing::error!("Order subscriber failed: {e:?}");
        }
    });
}

async fn subscribe(nats_client: async_nats::Client, provider: &AppProvider) -> anyhow::Result<()> {
    let mut subscriber = nats_client.subscribe("orders.created").await?;
    tracing::info!("Subscribed to orders.created");

    while let Some(message) = subscriber.next().await {
        let service = provider.audit_log_service();
        match serde_json::from_slice::<OrderEvent>(&message.payload) {
            Ok(event) => {
                let payload = match serde_json::to_value(&event) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!("Failed to serialize order event to JSON value: {e:?}");
                        serde_json::Value::Null
                    }
                };
                let audit_log = AuditLog {
                    id: AuditLogId::new(),
                    event_type: EventType::OrderCreated,
                    payload,
                    created_at: Utc::now(),
                };
                if let Err(e) = service.create(&audit_log).await {
                    tracing::error!("Failed to save audit log: {e:?}");
                }
            }
            Err(e) => {
                tracing::warn!("Failed to deserialize order event: {e:?}");
            }
        }
    }

    Ok(())
}
