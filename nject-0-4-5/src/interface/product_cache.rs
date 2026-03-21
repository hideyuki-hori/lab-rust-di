use async_trait::async_trait;

use crate::domain::product::Product;
use crate::domain::value_objects::ProductId;
use crate::error::AppError;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ProductCache: Send + Sync {
    async fn get_all(&self) -> Result<Option<Vec<Product>>, AppError>;
    async fn set_all(&self, products: &[Product]) -> Result<(), AppError>;
    async fn get_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError>;
    async fn set_by_id(&self, product: &Product) -> Result<(), AppError>;
    async fn invalidate(&self) -> Result<(), AppError>;
}
