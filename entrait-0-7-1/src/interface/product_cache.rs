use async_trait::async_trait;
use entrait::entrait;

use crate::domain::product::Product;
use crate::domain::value_objects::ProductId;
use crate::error::AppError;

#[entrait(mock_api = ProductCacheMock)]
#[async_trait]
pub trait ProductCache {
    async fn cache_get_all(&self) -> Result<Option<Vec<Product>>, AppError>;
    async fn cache_set_all(&self, products: &[Product]) -> Result<(), AppError>;
    async fn cache_get_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError>;
    async fn cache_set_by_id(&self, product: &Product) -> Result<(), AppError>;
    async fn cache_invalidate(&self) -> Result<(), AppError>;
}
