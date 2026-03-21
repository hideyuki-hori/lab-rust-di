use async_trait::async_trait;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;

#[async_trait]
pub trait ProductService: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Product>, AppError>;
    async fn find_by_id(&self, id: ProductId) -> Result<Product, AppError>;
    async fn create(
        &self,
        name: ProductName,
        price: Price,
        stock: Quantity,
        description: ProductDescription,
    ) -> Result<Product, AppError>;
}
