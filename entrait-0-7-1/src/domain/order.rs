use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::value_objects::{
    OrderId, OrderStatus, ProductId, Quantity, ShippingFee, Subtotal, TaxAmount, TotalPrice,
};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: OrderId,
    pub product_id: ProductId,
    pub quantity: Quantity,
    pub subtotal: Subtotal,
    pub tax_amount: TaxAmount,
    pub shipping_fee: ShippingFee,
    pub total_price: TotalPrice,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderEvent {
    pub order_id: OrderId,
    pub product_id: ProductId,
    pub quantity: Quantity,
    pub subtotal: Subtotal,
    pub tax_amount: TaxAmount,
    pub shipping_fee: ShippingFee,
    pub total_price: TotalPrice,
    pub created_at: DateTime<Utc>,
}
