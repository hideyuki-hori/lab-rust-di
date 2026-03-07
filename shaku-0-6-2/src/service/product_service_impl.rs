use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;
use crate::interface::product_service::ProductService;

#[derive(Component)]
#[shaku(interface = ProductService)]
pub struct ProductServiceImpl {
    #[shaku(inject)]
    repository: Arc<dyn ProductRepository>,
    #[shaku(inject)]
    cache: Arc<dyn ProductCache>,
}

#[async_trait]
impl ProductService for ProductServiceImpl {
    async fn find_all(&self) -> Result<Vec<Product>, AppError> {
        if let Some(cached) = self.cache.get_all().await? {
            return Ok(cached);
        }
        let products = self.repository.find_all().await?;
        let _ = self.cache.set_all(&products).await;
        Ok(products)
    }

    async fn find_by_id(&self, id: ProductId) -> Result<Product, AppError> {
        if let Some(cached) = self.cache.get_by_id(id).await? {
            return Ok(cached);
        }
        let product = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {id} not found")))?;
        let _ = self.cache.set_by_id(&product).await;
        Ok(product)
    }

    async fn create(
        &self,
        name: ProductName,
        price: Price,
        stock: Quantity,
        description: ProductDescription,
    ) -> Result<Product, AppError> {
        let now = Utc::now();
        let product = Product {
            id: ProductId::new(),
            name,
            price,
            stock,
            description,
            created_at: now,
            updated_at: now,
        };
        let created = self.repository.create(&product).await?;
        let _ = self.cache.invalidate().await;
        Ok(created)
    }
}
