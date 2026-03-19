use async_trait::async_trait;
use entrait::entrait;

use crate::domain::order::Order;
use crate::domain::value_objects::OrderId;
use crate::error::AppError;

#[entrait(mock_api = OrderRepositoryMock)]
#[async_trait]
pub trait OrderRepository {
    async fn create_order_record(&self, order: &Order) -> Result<Order, AppError>;
    async fn find_order_by_id(&self, id: OrderId) -> Result<Option<Order>, AppError>;
}
