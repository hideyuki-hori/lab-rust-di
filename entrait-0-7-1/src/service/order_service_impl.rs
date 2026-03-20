use chrono::Utc;
use entrait::entrait;

use crate::domain::order::{Order, OrderEvent};
use crate::domain::value_objects::{
    OrderId, OrderStatus, ProductId, Quantity, ShippingFee, TaxRate, TotalPrice,
};
use crate::error::AppError;
use crate::interface::event_publisher::EventPublisher;
use crate::interface::master_data_repository::MasterDataRepository;
use crate::interface::order_repository::OrderRepository;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;

#[entrait(pub OrderService, mock_api = OrderServiceMock)]
mod order_service {
    use super::*;

    pub async fn create_order(
        deps: &(impl OrderRepository
              + ProductRepository
              + ProductCache
              + EventPublisher
              + MasterDataRepository),
        product_id: ProductId,
        quantity: Quantity,
    ) -> Result<Order, AppError> {
        let max_qty: i32 = deps
            .get_master_data("max_order_quantity")
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Master data 'max_order_quantity' not configured".to_string())
            })?
            .parse()
            .map_err(|_| {
                AppError::Conflict("Invalid master data value for 'max_order_quantity'".to_string())
            })?;
        if quantity.0 > max_qty {
            return Err(AppError::Conflict(format!(
                "Order quantity exceeds maximum: max={max_qty}, requested={quantity}"
            )));
        }

        let product = deps
            .find_product_by_id(product_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {product_id} not found")))?;

        if product.stock < quantity {
            return Err(AppError::Conflict(format!(
                "Insufficient stock: available={}, requested={quantity}",
                product.stock
            )));
        }

        let tax_rate_value: f64 = deps
            .get_master_data("tax_rate")
            .await?
            .ok_or_else(|| AppError::NotFound("Master data 'tax_rate' not configured".to_string()))?
            .parse()
            .map_err(|_| {
                AppError::Conflict("Invalid master data value for 'tax_rate'".to_string())
            })?;
        let tax_rate = TaxRate::new(tax_rate_value).map_err(AppError::Conflict)?;

        let shipping_fee_value: i64 = deps
            .get_master_data("shipping_fee")
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Master data 'shipping_fee' not configured".to_string())
            })?
            .parse()
            .map_err(|_| {
                AppError::Conflict("Invalid master data value for 'shipping_fee'".to_string())
            })?;
        let shipping_fee = ShippingFee(shipping_fee_value);

        let subtotal = product.price * quantity;
        let tax_amount = subtotal.apply_rate(tax_rate);
        let total_price = TotalPrice::new(subtotal, tax_amount, shipping_fee);

        let now = Utc::now();
        let order = Order {
            id: OrderId::new(),
            product_id,
            quantity,
            subtotal,
            tax_amount,
            shipping_fee,
            total_price,
            status: OrderStatus::pending(),
            created_at: now,
        };

        let created = deps.create_order_record(&order).await?;

        deps.update_product_stock(product_id, -quantity).await?;
        let _ = deps.cache_invalidate().await;

        let event = OrderEvent {
            order_id: created.id,
            product_id,
            quantity,
            subtotal,
            tax_amount,
            shipping_fee,
            total_price,
            created_at: now,
        };
        let _ = deps.publish_order_created(&event).await;

        Ok(created)
    }

    pub async fn find_order_by_id_service(
        deps: &impl OrderRepository,
        id: OrderId,
    ) -> Result<Order, AppError> {
        deps.find_order_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Order {id} not found")))
    }
}

#[cfg(test)]
mod tests {
    use unimock::*;

    use super::order_service::OrderService;
    use crate::domain::order::Order;
    use crate::domain::value_objects::{
        OrderId, ProductId, Quantity, ShippingFee, Subtotal, TaxAmount, TotalPrice,
    };
    use crate::error::AppError;
    use crate::interface::event_publisher::EventPublisherMock;
    use crate::interface::master_data_repository::MasterDataRepositoryMock;
    use crate::interface::order_repository::OrderRepositoryMock;
    use crate::interface::product_cache::ProductCacheMock;
    use crate::interface::product_repository::ProductRepositoryMock;
    use crate::test_support::fixtures::{sample_order, sample_product_with};

    fn default_master_data_clauses() -> impl Clause {
        (
            MasterDataRepositoryMock::get_master_data
                .each_call(matching!("max_order_quantity"))
                .returns(Ok(Some("99".to_string()))),
            MasterDataRepositoryMock::get_master_data
                .each_call(matching!("tax_rate"))
                .returns(Ok(Some("0.10".to_string()))),
            MasterDataRepositoryMock::get_master_data
                .each_call(matching!("shipping_fee"))
                .returns(Ok(Some("500".to_string()))),
        )
    }

    fn product_repository_clauses(product_id: ProductId, price: i64, stock: i32) -> impl Clause {
        let product = sample_product_with(product_id, "Test Product", price, stock);
        (
            ProductRepositoryMock::find_product_by_id
                .each_call(matching!(_))
                .returns(Ok(Some(product))),
            ProductRepositoryMock::update_product_stock
                .each_call(matching!(_, _))
                .returns(Ok(())),
        )
    }

    fn order_record_clause() -> impl Clause {
        OrderRepositoryMock::create_order_record
            .each_call(matching!(_))
            .answers(&|_, order: &Order| Ok(order.clone()))
    }

    #[tokio::test]
    async fn create_order_success() {
        let product_id = ProductId::new();

        let mock = Unimock::new_partial((
            default_master_data_clauses(),
            product_repository_clauses(product_id, 1000, 10),
            ProductCacheMock::cache_invalidate
                .each_call(matching!())
                .returns(Ok(())),
            order_record_clause(),
            EventPublisherMock::publish_order_created
                .each_call(matching!(_))
                .returns(Ok(())),
        ));

        let order = mock.create_order(product_id, Quantity(2)).await.unwrap();
        assert_eq!(order.product_id, product_id);
        assert_eq!(order.quantity, Quantity(2));
    }

    #[tokio::test]
    async fn create_order_exceeds_max_quantity() {
        let product_id = ProductId::new();

        let mock = Unimock::new_partial(
            MasterDataRepositoryMock::get_master_data
                .each_call(matching!("max_order_quantity"))
                .returns(Ok(Some("5".to_string()))),
        );

        let result = mock.create_order(product_id, Quantity(10)).await;
        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn create_order_product_not_found() {
        let product_id = ProductId::new();

        let mock = Unimock::new_partial((
            default_master_data_clauses(),
            ProductRepositoryMock::find_product_by_id
                .each_call(matching!(_))
                .returns(Ok(None)),
        ));

        let result = mock.create_order(product_id, Quantity(1)).await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn create_order_insufficient_stock() {
        let product_id = ProductId::new();
        let product = sample_product_with(product_id, "Test Product", 1000, 3);

        let mock = Unimock::new_partial((
            default_master_data_clauses(),
            ProductRepositoryMock::find_product_by_id
                .each_call(matching!(_))
                .returns(Ok(Some(product))),
        ));

        let result = mock.create_order(product_id, Quantity(5)).await;
        assert!(matches!(result, Err(AppError::Conflict(_))));
    }

    #[tokio::test]
    async fn create_order_calculates_correct_totals() {
        let product_id = ProductId::new();

        let mock = Unimock::new_partial((
            default_master_data_clauses(),
            product_repository_clauses(product_id, 1000, 10),
            ProductCacheMock::cache_invalidate
                .each_call(matching!())
                .returns(Ok(())),
            order_record_clause(),
            EventPublisherMock::publish_order_created
                .each_call(matching!(_))
                .returns(Ok(())),
        ));

        let order = mock.create_order(product_id, Quantity(3)).await.unwrap();
        assert_eq!(order.subtotal, Subtotal(3000));
        assert_eq!(order.tax_amount, TaxAmount(300));
        assert_eq!(order.shipping_fee, ShippingFee(500));
        assert_eq!(order.total_price, TotalPrice(3800));
    }

    #[tokio::test]
    async fn find_by_id_success() {
        let product_id = ProductId::new();
        let order = sample_order(product_id);
        let order_id = order.id;
        let expected = order.clone();

        let mock = Unimock::new_partial(
            OrderRepositoryMock::find_order_by_id
                .each_call(matching!(_))
                .returns(Ok(Some(expected))),
        );

        let result = mock.find_order_by_id_service(order_id).await.unwrap();
        assert_eq!(result.id, order_id);
        assert_eq!(result.product_id, product_id);
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let order_id = OrderId::new();

        let mock = Unimock::new_partial(
            OrderRepositoryMock::find_order_by_id
                .each_call(matching!(_))
                .returns(Ok(None)),
        );

        let result = mock.find_order_by_id_service(order_id).await;
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }
}
