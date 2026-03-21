use async_trait::async_trait;

use crate::domain::order::Order;
use crate::domain::value_objects::{OrderId, ProductId, Quantity};
use crate::error::AppError;

#[async_trait]
pub trait OrderService: Send + Sync {
    async fn create_order(
        &self,
        product_id: ProductId,
        quantity: Quantity,
    ) -> Result<Order, AppError>;
    async fn find_by_id(&self, id: OrderId) -> Result<Order, AppError>;
}
