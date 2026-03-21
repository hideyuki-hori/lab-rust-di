use mockall::predicate::*;

use crate::domain::value_objects::{Price, ProductId, ProductName, Quantity};
use crate::interface::product_cache::MockProductCache;
use crate::interface::product_repository::MockProductRepository;
use crate::interface::product_service::ProductService;
use crate::service::product_service_impl::ProductServiceImpl;
use crate::test_support::fixtures::{sample_product, sample_product_with};

#[tokio::test]
async fn find_all_returns_cached() {
    let products = vec![sample_product()];
    let cached = products.clone();

    let mut cache = MockProductCache::new();
    cache
        .expect_get_all()
        .times(1)
        .returning(move || Ok(Some(cached.clone())));

    let mut repo = MockProductRepository::new();
    repo.expect_find_all().never();

    let service = ProductServiceImpl {
        repository: &repo,
        cache: &cache,
    };

    let result = service.find_all().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, products[0].id);
}

#[tokio::test]
async fn find_all_fetches_from_repository_on_cache_miss() {
    let products = vec![sample_product()];
    let from_repository = products.clone();

    let mut cache = MockProductCache::new();
    cache.expect_get_all().times(1).returning(|| Ok(None));
    cache.expect_set_all().times(1).returning(|_| Ok(()));

    let mut repo = MockProductRepository::new();
    repo.expect_find_all()
        .times(1)
        .returning(move || Ok(from_repository.clone()));

    let service = ProductServiceImpl {
        repository: &repo,
        cache: &cache,
    };

    let result = service.find_all().await.unwrap();
    assert_eq!(result.len(), 1);
}

#[tokio::test]
async fn find_by_id_cache_hit() {
    let product = sample_product();
    let id = product.id;
    let cached = product.clone();

    let mut cache = MockProductCache::new();
    cache
        .expect_get_by_id()
        .with(eq(id))
        .times(1)
        .returning(move |_| Ok(Some(cached.clone())));

    let mut repo = MockProductRepository::new();
    repo.expect_find_by_id().never();

    let service = ProductServiceImpl {
        repository: &repo,
        cache: &cache,
    };

    let result = service.find_by_id(id).await.unwrap();
    assert_eq!(result.id, id);
}

#[tokio::test]
async fn find_by_id_not_found() {
    let id = ProductId::new();

    let mut cache = MockProductCache::new();
    cache
        .expect_get_by_id()
        .with(eq(id))
        .times(1)
        .returning(|_| Ok(None));

    let mut repo = MockProductRepository::new();
    repo.expect_find_by_id()
        .with(eq(id))
        .times(1)
        .returning(|_| Ok(None));

    let service = ProductServiceImpl {
        repository: &repo,
        cache: &cache,
    };

    let result = service.find_by_id(id).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, crate::error::AppError::NotFound(_)));
}

#[tokio::test]
async fn create_invalidates_cache() {
    let product = sample_product_with(ProductId::new(), "New Product", 500, 20);
    let created = product.clone();

    let mut cache = MockProductCache::new();
    cache.expect_invalidate().times(1).returning(|| Ok(()));

    let mut repo = MockProductRepository::new();
    repo.expect_create()
        .times(1)
        .returning(move |_| Ok(created.clone()));

    let service = ProductServiceImpl {
        repository: &repo,
        cache: &cache,
    };

    let result = service
        .create(
            ProductName::new("New Product").unwrap(),
            Price::new(500).unwrap(),
            Quantity::new(20).unwrap(),
            Default::default(),
        )
        .await
        .unwrap();
    assert_eq!(result.name.to_string(), "New Product");
}
