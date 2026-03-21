use async_trait::async_trait;
use chrono::Utc;
use nject::injectable;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;
use crate::interface::product_service::ProductService;

#[injectable]
pub(crate) struct ProductServiceImpl<'a> {
    pub(crate) repository: &'a dyn ProductRepository,
    pub(crate) cache: &'a dyn ProductCache,
}

#[async_trait]
impl ProductService for ProductServiceImpl<'_> {
    async fn find_all(&self) -> Result<Vec<Product>, AppError> {
        if let Some(cached) = self.cache.get_all().await? {
            return Ok(cached);
        }
        let products = self.repository.find_all().await?;
        if let Err(e) = self.cache.set_all(&products).await {
            tracing::warn!("Cache set_all failed: {e}");
        }
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
        if let Err(e) = self.cache.set_by_id(&product).await {
            tracing::warn!("Cache set_by_id failed: {e}");
        }
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
        if let Err(e) = self.cache.invalidate().await {
            tracing::warn!("Cache invalidation failed: {e}");
        }
        Ok(created)
    }
}
