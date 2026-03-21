use async_trait::async_trait;
use chrono::Utc;
use nject::injectable;

use crate::domain::order::{Order, OrderEvent};
use crate::domain::value_objects::{
    OrderId, OrderStatus, ProductId, Quantity, ShippingFee, TaxRate, TotalPrice,
};
use crate::error::AppError;
use crate::interface::event_publisher::EventPublisher;
use crate::interface::master_data_repository::MasterDataRepository;
use crate::interface::order_repository::OrderRepository;
use crate::interface::order_service::OrderService;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;

#[injectable]
pub(crate) struct OrderServiceImpl<'a> {
    pub(crate) order_repository: &'a dyn OrderRepository,
    pub(crate) product_repository: &'a dyn ProductRepository,
    pub(crate) product_cache: &'a dyn ProductCache,
    pub(crate) event_publisher: &'a dyn EventPublisher,
    pub(crate) master_data: &'a dyn MasterDataRepository,
}

#[async_trait]
impl OrderService for OrderServiceImpl<'_> {
    async fn create_order(
        &self,
        product_id: ProductId,
        quantity: Quantity,
    ) -> Result<Order, AppError> {
        let max_qty: i32 = self
            .master_data
            .get("max_order_quantity")
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Master data 'max_order_quantity' not configured".to_string())
            })?
            .parse()
            .map_err(|_| {
                AppError::BadRequest(
                    "Invalid master data value for 'max_order_quantity'".to_string(),
                )
            })?;
        if quantity.0 > max_qty {
            return Err(AppError::Conflict(format!(
                "Order quantity exceeds maximum: max={max_qty}, requested={quantity}"
            )));
        }

        let product = self
            .product_repository
            .find_by_id(product_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Product {product_id} not found")))?;

        if product.stock < quantity {
            return Err(AppError::Conflict(format!(
                "Insufficient stock: available={}, requested={quantity}",
                product.stock
            )));
        }

        let tax_rate_value: f64 = self
            .master_data
            .get("tax_rate")
            .await?
            .ok_or_else(|| AppError::NotFound("Master data 'tax_rate' not configured".to_string()))?
            .parse()
            .map_err(|_| {
                AppError::BadRequest("Invalid master data value for 'tax_rate'".to_string())
            })?;
        let tax_rate = TaxRate::new(tax_rate_value).map_err(AppError::BadRequest)?;

        let shipping_fee_value: i64 = self
            .master_data
            .get("shipping_fee")
            .await?
            .ok_or_else(|| {
                AppError::NotFound("Master data 'shipping_fee' not configured".to_string())
            })?
            .parse()
            .map_err(|_| {
                AppError::BadRequest("Invalid master data value for 'shipping_fee'".to_string())
            })?;
        let shipping_fee = ShippingFee::new(shipping_fee_value).map_err(AppError::BadRequest)?;

        let subtotal = product.price * quantity;
        let tax_amount = subtotal.apply_rate(tax_rate);
        let total_price = TotalPrice::calculate(subtotal, tax_amount, shipping_fee)
            .map_err(AppError::BadRequest)?;

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

        let created = self.order_repository.create(&order).await?;

        self.product_repository
            .update_stock(product_id, quantity.as_negative_i32())
            .await?;
        if let Err(e) = self.product_cache.invalidate().await {
            tracing::warn!("Cache invalidation failed: {e}");
        }

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
        if let Err(e) = self.event_publisher.publish_order_created(&event).await {
            tracing::warn!("Failed to publish order created event: {e}");
        }

        Ok(created)
    }

    async fn find_by_id(&self, id: OrderId) -> Result<Order, AppError> {
        self.order_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Order {id} not found")))
    }
}
