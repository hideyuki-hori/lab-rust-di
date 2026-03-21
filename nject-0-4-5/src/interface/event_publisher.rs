use async_trait::async_trait;

use crate::domain::order::OrderEvent;
use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish_order_created(&self, event: &OrderEvent) -> Result<(), AppError>;
}
