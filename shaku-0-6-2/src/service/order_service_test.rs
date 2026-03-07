use mockall::predicate::*;
use shaku::HasComponent;

use crate::domain::value_objects::{OrderId, ProductId, Quantity, Subtotal, TaxAmount, ShippingFee, TotalPrice};
use crate::error::AppError;
use crate::interface::event_publisher::MockEventPublisher;
use crate::interface::master_data_repository::MockMasterDataRepository;
use crate::interface::order_repository::MockOrderRepository;
use crate::interface::order_service::OrderService;
use crate::interface::product_cache::MockProductCache;
use crate::interface::product_repository::MockProductRepository;
use crate::test_support::fixtures::{sample_order, sample_product_with};
use crate::test_support::test_module::TestModuleBuilder;

fn default_master_data() -> MockMasterDataRepository {
    let mut master = MockMasterDataRepository::new();
    master
        .expect_get()
        .with(eq("max_order_quantity"))
        .returning(|_| Ok(Some("99".to_string())));
    master
        .expect_get()
        .with(eq("tax_rate"))
        .returning(|_| Ok(Some("0.10".to_string())));
    master
        .expect_get()
        .with(eq("shipping_fee"))
        .returning(|_| Ok(Some("500".to_string())));
    master
}

fn product_repository_with_product(product_id: ProductId, price: i64, stock: i32) -> MockProductRepository {
    let product = sample_product_with(product_id, "Test Product", price, stock);
    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_by_id()
        .with(eq(product_id))
        .returning(move |_| Ok(Some(product.clone())));
    product_repository.expect_update_stock()
        .returning(|_, _| Ok(()));
    product_repository
}

#[tokio::test]
async fn create_order_success() {
    let product_id = ProductId::new();

    let product_repository = product_repository_with_product(product_id, 1000, 10);

    let mut cache = MockProductCache::new();
    cache.expect_invalidate().returning(|| Ok(()));

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_create()
        .times(1)
        .returning(|order| Ok(order.clone()));

    let mut event_pub = MockEventPublisher::new();
    event_pub
        .expect_publish_order_created()
        .times(1)
        .returning(|_| Ok(()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .with_order_repository(order_repository)
        .with_event_publisher(event_pub)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let order = svc.create_order(product_id, Quantity(2)).await.unwrap();
    assert_eq!(order.product_id, product_id);
    assert_eq!(order.quantity, Quantity(2));
}

#[tokio::test]
async fn create_order_exceeds_max_quantity() {
    let product_id = ProductId::new();

    let mut master = MockMasterDataRepository::new();
    master
        .expect_get()
        .with(eq("max_order_quantity"))
        .returning(|_| Ok(Some("5".to_string())));

    let module = TestModuleBuilder::new()
        .with_master_data(master)
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let result = svc.create_order(product_id, Quantity(10)).await;
    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn create_order_product_not_found() {
    let product_id = ProductId::new();

    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_by_id()
        .with(eq(product_id))
        .returning(|_| Ok(None));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let result = svc.create_order(product_id, Quantity(1)).await;
    assert!(matches!(result, Err(AppError::NotFound(_))));
}

#[tokio::test]
async fn create_order_insufficient_stock() {
    let product_id = ProductId::new();

    let product_repository = product_repository_with_product(product_id, 1000, 3);

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let result = svc.create_order(product_id, Quantity(5)).await;
    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn create_order_calculates_correct_totals() {
    let product_id = ProductId::new();

    let product_repository = product_repository_with_product(product_id, 1000, 10);

    let mut cache = MockProductCache::new();
    cache.expect_invalidate().returning(|| Ok(()));

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_create()
        .times(1)
        .returning(|order| Ok(order.clone()));

    let mut event_pub = MockEventPublisher::new();
    event_pub
        .expect_publish_order_created()
        .returning(|_| Ok(()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .with_order_repository(order_repository)
        .with_event_publisher(event_pub)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let order = svc.create_order(product_id, Quantity(3)).await.unwrap();
    assert_eq!(order.subtotal, Subtotal(3000));
    assert_eq!(order.tax_amount, TaxAmount(300));
    assert_eq!(order.shipping_fee, ShippingFee(500));
    assert_eq!(order.total_price, TotalPrice(3800));
}

#[tokio::test]
async fn create_order_publishes_event() {
    let product_id = ProductId::new();

    let product_repository = product_repository_with_product(product_id, 1000, 10);

    let mut cache = MockProductCache::new();
    cache.expect_invalidate().returning(|| Ok(()));

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_create()
        .returning(|order| Ok(order.clone()));

    let mut event_pub = MockEventPublisher::new();
    event_pub
        .expect_publish_order_created()
        .times(1)
        .withf(move |event| event.product_id == product_id && event.quantity == Quantity(2))
        .returning(|_| Ok(()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .with_order_repository(order_repository)
        .with_event_publisher(event_pub)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    svc.create_order(product_id, Quantity(2)).await.unwrap();
}

#[tokio::test]
async fn create_order_updates_stock() {
    let product_id = ProductId::new();

    let product = sample_product_with(product_id, "Test Product", 1000, 10);
    let mut product_repository = MockProductRepository::new();
    product_repository.expect_find_by_id()
        .with(eq(product_id))
        .returning(move |_| Ok(Some(product.clone())));
    product_repository.expect_update_stock()
        .times(1)
        .withf(move |id, delta| *id == product_id && *delta == Quantity(-3))
        .returning(|_, _| Ok(()));

    let mut cache = MockProductCache::new();
    cache.expect_invalidate().returning(|| Ok(()));

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_create()
        .returning(|order| Ok(order.clone()));

    let mut event_pub = MockEventPublisher::new();
    event_pub
        .expect_publish_order_created()
        .returning(|_| Ok(()));

    let module = TestModuleBuilder::new()
        .with_product_repository(product_repository)
        .with_product_cache(cache)
        .with_order_repository(order_repository)
        .with_event_publisher(event_pub)
        .with_master_data(default_master_data())
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    svc.create_order(product_id, Quantity(3)).await.unwrap();
}

#[tokio::test]
async fn find_by_id_success() {
    let product_id = ProductId::new();
    let order = sample_order(product_id);
    let order_id = order.id;
    let expected = order.clone();

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_find_by_id()
        .with(eq(order_id))
        .times(1)
        .returning(move |_| Ok(Some(expected.clone())));

    let module = TestModuleBuilder::new()
        .with_order_repository(order_repository)
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let result = svc.find_by_id(order_id).await.unwrap();
    assert_eq!(result.id, order_id);
    assert_eq!(result.product_id, product_id);
}

#[tokio::test]
async fn find_by_id_not_found() {
    let order_id = OrderId::new();

    let mut order_repository = MockOrderRepository::new();
    order_repository
        .expect_find_by_id()
        .with(eq(order_id))
        .times(1)
        .returning(|_| Ok(None));

    let module = TestModuleBuilder::new()
        .with_order_repository(order_repository)
        .build();
    let svc: &dyn OrderService = module.resolve_ref();

    let result = svc.find_by_id(order_id).await;
    assert!(matches!(result, Err(AppError::NotFound(_))));
}
