use async_trait::async_trait;
use shaku::Interface;

use crate::domain::product::Product;
use crate::domain::value_objects::{ProductId, Quantity};
use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProductRepository: Interface {
    async fn find_all(&self) -> Result<Vec<Product>, AppError>;
    async fn find_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError>;
    async fn create(&self, product: &Product) -> Result<Product, AppError>;
    async fn update_stock(&self, id: ProductId, delta: Quantity) -> Result<(), AppError>;
}
