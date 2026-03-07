use async_trait::async_trait;
use shaku::Component;

use crate::domain::order::OrderEvent;
use crate::error::AppError;
use crate::interface::event_publisher::EventPublisher;

#[derive(Component)]
#[shaku(interface = EventPublisher)]
pub struct NatsEventPublisher {
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
