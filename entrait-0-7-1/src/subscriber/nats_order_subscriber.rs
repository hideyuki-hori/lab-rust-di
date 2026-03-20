use std::sync::Arc;

use chrono::Utc;
use tokio_stream::StreamExt;

use crate::domain::audit_log::AuditLog;
use crate::domain::order::OrderEvent;
use crate::domain::value_objects::AuditLogId;
use crate::service::audit_log_service_impl::AuditLogService;

pub fn spawn<S: AuditLogService + Send + Sync + 'static>(
    nats_client: async_nats::Client,
    app: Arc<S>,
) {
    tokio::spawn(async move {
        if let Err(e) = subscribe(nats_client, app).await {
            tracing::error!("Order subscriber failed: {e:?}");
        }
    });
}

async fn subscribe<S: AuditLogService + Send + Sync>(
    nats_client: async_nats::Client,
    app: Arc<S>,
) -> anyhow::Result<()> {
    let mut subscriber = nats_client.subscribe("orders.created").await?;
    tracing::info!("Subscribed to orders.created");

    while let Some(message) = subscriber.next().await {
        match serde_json::from_slice::<OrderEvent>(&message.payload) {
            Ok(event) => {
                let audit_log = AuditLog {
                    id: AuditLogId::new(),
                    event_type: "order.created".to_string(),
                    payload: serde_json::to_value(&event).unwrap_or_default(),
                    created_at: Utc::now(),
                };
                if let Err(e) = app.create_audit_log_svc(&audit_log).await {
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
