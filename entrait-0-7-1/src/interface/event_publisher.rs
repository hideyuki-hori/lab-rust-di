use async_trait::async_trait;
use entrait::entrait;

use crate::domain::order::OrderEvent;
use crate::error::AppError;

#[entrait(mock_api = EventPublisherMock)]
#[async_trait]
pub trait EventPublisher {
    async fn publish_order_created(&self, event: &OrderEvent) -> Result<(), AppError>;
}
