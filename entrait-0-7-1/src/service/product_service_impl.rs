use chrono::Utc;
use entrait::entrait;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;

#[entrait(pub ProductService, mock_api = ProductServiceMock)]
mod product_service {
    use super::*;

    pub async fn find_all(
        deps: &(impl ProductRepository + ProductCache),
    ) -> Result<Vec<Product>, AppError> {
        if let Some(cached) = deps.cache_get_all().await? {
            return Ok(cached);
        }
        let products = deps.find_all_products().await?;
        let _ = deps.cache_set_all(&products).await;
        Ok(products)
    }

    pub async fn find_by_id(
        deps: &(impl ProductRepository + ProductCache),
        id: ProductId,
    ) -> Result<Product, AppError> {
        if let Some(cached) = deps.cache_get_by_id(id).await? {
            return Ok(cached);
        }
        let product = deps
            .find_product_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {id} not found")))?;
        let _ = deps.cache_set_by_id(&product).await;
        Ok(product)
    }

    pub async fn create(
        deps: &(impl ProductRepository + ProductCache),
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
        let created = deps.create_product(&product).await?;
        let _ = deps.cache_invalidate().await;
        Ok(created)
    }
}

#[cfg(test)]
mod tests {
    use unimock::*;

    use super::product_service::ProductService;
    use crate::domain::value_objects::{Price, ProductId, ProductName, Quantity};
    use crate::interface::product_cache::ProductCacheMock;
    use crate::interface::product_repository::ProductRepositoryMock;
    use crate::test_support::fixtures::{sample_product, sample_product_with};

    #[tokio::test]
    async fn find_all_returns_cached() {
        let products = vec![sample_product()];
        let cached = products.clone();

        let mock = Unimock::new_partial(
            ProductCacheMock::cache_get_all
                .each_call(matching!())
                .returns(Ok(Some(cached))),
        );

        let result = mock.find_all().await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, products[0].id);
    }

    #[tokio::test]
    async fn find_all_fetches_from_repository_on_cache_miss() {
        let products = vec![sample_product()];
        let from_repository = products.clone();

        let mock = Unimock::new_partial((
            ProductCacheMock::cache_get_all
                .each_call(matching!())
                .returns(Ok(None)),
            ProductRepositoryMock::find_all_products
                .each_call(matching!())
                .returns(Ok(from_repository)),
            ProductCacheMock::cache_set_all
                .each_call(matching!(_))
                .returns(Ok(())),
        ));

        let result = mock.find_all().await.unwrap();
        assert_eq!(result.len(), 1);
    }

    #[tokio::test]
    async fn find_by_id_cache_hit() {
        let product = sample_product();
        let id = product.id;
        let cached = product.clone();

        let mock = Unimock::new_partial(
            ProductCacheMock::cache_get_by_id
                .each_call(matching!(_))
                .returns(Ok(Some(cached))),
        );

        let result = mock.find_by_id(id).await.unwrap();
        assert_eq!(result.id, id);
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let id = ProductId::new();

        let mock = Unimock::new_partial((
            ProductCacheMock::cache_get_by_id
                .each_call(matching!(_))
                .returns(Ok(None)),
            ProductRepositoryMock::find_product_by_id
                .each_call(matching!(_))
                .returns(Ok(None)),
        ));

        let result = mock.find_by_id(id).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::error::AppError::NotFound(_)
        ));
    }

    #[tokio::test]
    async fn create_invalidates_cache() {
        let product = sample_product_with(ProductId::new(), "New Product", 500, 20);
        let created = product.clone();

        let mock = Unimock::new_partial((
            ProductRepositoryMock::create_product
                .each_call(matching!(_))
                .returns(Ok(created)),
            ProductCacheMock::cache_invalidate
                .each_call(matching!())
                .returns(Ok(())),
        ));

        let result = mock
            .create(
                ProductName::new("New Product").unwrap(),
                Price(500),
                Quantity(20),
                Default::default(),
            )
            .await
            .unwrap();
        assert_eq!(result.name.to_string(), "New Product");
    }
}
