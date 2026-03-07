use mockall::predicate::*;
use shaku::HasComponent;

use crate::domain::value_objects::{Price, ProductId, ProductName, Quantity};
use crate::interface::product_cache::MockProductCache;
use crate::interface::product_repository::MockProductRepository;
use crate::interface::product_service::ProductService;
use crate::test_support::fixtures::{sample_product, sample_product_with};
use crate::test_support::test_module::TestModuleBuilder;

#[tokio::test]
async fn find_all_returns_cached() {
    let products = vec![sample_product()];
    let cached = products.clone();

    let mut cache = MockProductCache::new();
    cache
        .expect_get_all()
        .times(1)
        .returning(move || Ok(Some(cached.clone())));

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_all().never();

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .build();
    let svc: &dyn ProductService = module.resolve_ref();

    let result = svc.find_all().await.unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, products[0].id);
}

#[tokio::test]
async fn find_all_fetches_from_repository_on_cache_miss() {
    let products = vec![sample_product()];
    let from_repository = products.clone();

    let mut cache = MockProductCache::new();
    cache
        .expect_get_all()
        .times(1)
        .returning(|| Ok(None));
    cache
        .expect_set_all()
        .times(1)
        .returning(|_| Ok(()));

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_all()
        .times(1)
        .returning(move || Ok(from_repository.clone()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .build();
    let svc: &dyn ProductService = module.resolve_ref();

    let result = svc.find_all().await.unwrap();
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

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_by_id().never();

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .build();
    let svc: &dyn ProductService = module.resolve_ref();

    let result = svc.find_by_id(id).await.unwrap();
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

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_by_id()
        .with(eq(id))
        .times(1)
        .returning(|_| Ok(None));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .build();
    let svc: &dyn ProductService = module.resolve_ref();

    let result = svc.find_by_id(id).await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(matches!(err, crate::error::AppError::NotFound(_)));
}

#[tokio::test]
async fn create_invalidates_cache() {
    let product = sample_product_with(
        ProductId::new(),
        "New Product",
        500,
        20,
    );
    let created = product.clone();

    let mut cache = MockProductCache::new();
    cache
        .expect_invalidate()
        .times(1)
        .returning(|| Ok(()));

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_create()
        .times(1)
        .returning(move |_| Ok(created.clone()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .build();
    let svc: &dyn ProductService = module.resolve_ref();

    let result = svc
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
