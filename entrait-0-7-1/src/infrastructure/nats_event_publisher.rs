use async_trait::async_trait;
use entrait::Impl;

use crate::app_state::AppState;
use crate::domain::order::OrderEvent;
use crate::error::AppError;
use crate::interface::event_publisher::EventPublisher;

#[async_trait]
impl EventPublisher for Impl<AppState> {
    async fn publish_order_created(&self, event: &OrderEvent) -> Result<(), AppError> {
        let payload = serde_json::to_vec(event)?;
        self.nats_client
            .publish("orders.created", payload.into())
            .await
            .map_err(|e| AppError::Internal(format!("NATS publish error: {e}")))?;
        Ok(())
    }
}
