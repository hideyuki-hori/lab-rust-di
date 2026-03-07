use async_trait::async_trait;
use shaku::Interface;

use crate::domain::order::Order;
use crate::domain::value_objects::OrderId;
use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait OrderRepository: Interface {
    async fn create(&self, order: &Order) -> Result<Order, AppError>;
    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, AppError>;
}
