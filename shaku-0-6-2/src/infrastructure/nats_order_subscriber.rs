use std::sync::Arc;

use chrono::Utc;
use tokio_stream::StreamExt;
use shaku::HasComponent;

use crate::domain::audit_log::AuditLog;
use crate::domain::order::OrderEvent;
use crate::domain::value_objects::AuditLogId;
use crate::interface::audit_log_repository::AuditLogRepository;
use crate::module::InfraModule;

pub fn spawn(nats_client: async_nats::Client, infra_module: Arc<InfraModule>) {
    tokio::spawn(async move {
        if let Err(e) = subscribe(nats_client, infra_module).await {
            tracing::error!("Order subscriber failed: {e:?}");
        }
    });
}

async fn subscribe(
    nats_client: async_nats::Client,
    infra_module: Arc<InfraModule>,
) -> anyhow::Result<()> {
    let mut subscriber = nats_client.subscribe("orders.created").await?;
    tracing::info!("Subscribed to orders.created");

    while let Some(message) = subscriber.next().await {
        let repository: Arc<dyn AuditLogRepository> = infra_module.resolve();
        match serde_json::from_slice::<OrderEvent>(&message.payload) {
            Ok(event) => {
                let audit_log = AuditLog {
                    id: AuditLogId::new(),
                    event_type: "order.created".to_string(),
                    payload: serde_json::to_value(&event).unwrap_or_default(),
                    created_at: Utc::now(),
                };
                if let Err(e) = repository.create(&audit_log).await {
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
