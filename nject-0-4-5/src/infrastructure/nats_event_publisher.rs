use async_trait::async_trait;
use nject::inject;

use crate::domain::order::OrderEvent;
use crate::error::AppError;
use crate::interface::event_publisher::EventPublisher;

#[inject(|client: &'prov async_nats::Client| Self { client: client.clone() })]
pub(crate) struct NatsEventPublisher {
    client: async_nats::Client,
}

#[async_trait]
impl EventPublisher for NatsEventPublisher {
    async fn publish_order_created(&self, event: &OrderEvent) -> Result<(), AppError> {
        let payload = serde_json::to_vec(event)?;
        self.client
            .publish("orders.created", payload.into())
            .await
            .map_err(|e| AppError::Internal(anyhow::anyhow!("NATS publish error: {e}")))?;
        Ok(())
    }
}
