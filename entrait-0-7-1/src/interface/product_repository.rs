use async_trait::async_trait;
use entrait::entrait;

use crate::domain::product::Product;
use crate::domain::value_objects::{ProductId, Quantity};
use crate::error::AppError;

#[entrait(mock_api = ProductRepositoryMock)]
#[async_trait]
pub trait ProductRepository {
    async fn find_all_products(&self) -> Result<Vec<Product>, AppError>;
    async fn find_product_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError>;
    async fn create_product(&self, product: &Product) -> Result<Product, AppError>;
    async fn update_product_stock(&self, id: ProductId, delta: Quantity) -> Result<(), AppError>;
}
