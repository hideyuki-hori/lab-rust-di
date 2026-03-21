use mockall::predicate::*;

use crate::domain::value_objects::{
    OrderId, ProductId, Quantity, ShippingFee, Subtotal, TaxAmount, TotalPrice,
};
use crate::error::AppError;
use crate::interface::event_publisher::MockEventPublisher;
use crate::interface::master_data_repository::MockMasterDataRepository;
use crate::interface::order_repository::MockOrderRepository;
use crate::interface::order_service::OrderService;
use crate::interface::product_cache::MockProductCache;
use crate::interface::product_repository::MockProductRepository;
use crate::service::order_service_impl::OrderServiceImpl;
use crate::test_support::fixtures::{sample_order, sample_product_with};

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

fn product_repository_with_product(
    product_id: ProductId,
    price: i64,
    stock: i32,
) -> MockProductRepository {
    let product = sample_product_with(product_id, "Test Product", price, stock);
    let mut product_repository = MockProductRepository::new();
    product_repository
        .expect_find_by_id()
        .with(eq(product_id))
        .returning(move |_| Ok(Some(product.clone())));
    product_repository
        .expect_update_stock()
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

    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let order = service
        .create_order(product_id, Quantity::new(2).unwrap())
        .await
        .unwrap();
    assert_eq!(order.product_id, product_id);
    assert_eq!(order.quantity, Quantity::new(2).unwrap());
}

#[tokio::test]
async fn create_order_exceeds_max_quantity() {
    let product_id = ProductId::new();

    let mut master = MockMasterDataRepository::new();
    master
        .expect_get()
        .with(eq("max_order_quantity"))
        .returning(|_| Ok(Some("5".to_string())));

    let order_repository = MockOrderRepository::new();
    let product_repository = MockProductRepository::new();
    let cache = MockProductCache::new();
    let event_pub = MockEventPublisher::new();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master,
    };

    let result = service
        .create_order(product_id, Quantity::new(10).unwrap())
        .await;
    assert!(matches!(result, Err(AppError::Conflict(_))));
}

#[tokio::test]
async fn create_order_product_not_found() {
    let product_id = ProductId::new();

    let mut product_repository = MockProductRepository::new();
    product_repository
        .expect_find_by_id()
        .with(eq(product_id))
        .returning(|_| Ok(None));

    let order_repository = MockOrderRepository::new();
    let cache = MockProductCache::new();
    let event_pub = MockEventPublisher::new();
    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let result = service
        .create_order(product_id, Quantity::new(1).unwrap())
        .await;
    assert!(matches!(result, Err(AppError::NotFound(_))));
}

#[tokio::test]
async fn create_order_insufficient_stock() {
    let product_id = ProductId::new();

    let product_repository = product_repository_with_product(product_id, 1000, 3);

    let order_repository = MockOrderRepository::new();
    let cache = MockProductCache::new();
    let event_pub = MockEventPublisher::new();
    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let result = service
        .create_order(product_id, Quantity::new(5).unwrap())
        .await;
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

    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let order = service
        .create_order(product_id, Quantity::new(3).unwrap())
        .await
        .unwrap();
    assert_eq!(order.subtotal, Subtotal::new(3000).unwrap());
    assert_eq!(order.tax_amount, TaxAmount::new(300).unwrap());
    assert_eq!(order.shipping_fee, ShippingFee::new(500).unwrap());
    assert_eq!(order.total_price, TotalPrice::new(3800).unwrap());
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
        .withf(move |event| {
            event.product_id == product_id && event.quantity == Quantity::new(2).unwrap()
        })
        .returning(|_| Ok(()));

    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    service
        .create_order(product_id, Quantity::new(2).unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn create_order_updates_stock() {
    let product_id = ProductId::new();

    let product = sample_product_with(product_id, "Test Product", 1000, 10);
    let mut product_repository = MockProductRepository::new();
    product_repository
        .expect_find_by_id()
        .with(eq(product_id))
        .returning(move |_| Ok(Some(product.clone())));
    product_repository
        .expect_update_stock()
        .times(1)
        .withf(move |id, delta| *id == product_id && *delta == -3)
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

    let master_data = default_master_data();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    service
        .create_order(product_id, Quantity::new(3).unwrap())
        .await
        .unwrap();
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

    let product_repository = MockProductRepository::new();
    let cache = MockProductCache::new();
    let event_pub = MockEventPublisher::new();
    let master_data = MockMasterDataRepository::new();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let result = service.find_by_id(order_id).await.unwrap();
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

    let product_repository = MockProductRepository::new();
    let cache = MockProductCache::new();
    let event_pub = MockEventPublisher::new();
    let master_data = MockMasterDataRepository::new();

    let service = OrderServiceImpl {
        order_repository: &order_repository,
        product_repository: &product_repository,
        product_cache: &cache,
        event_publisher: &event_pub,
        master_data: &master_data,
    };

    let result = service.find_by_id(order_id).await;
    assert!(matches!(result, Err(AppError::NotFound(_))));
}
