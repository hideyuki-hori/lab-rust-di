use chrono::Utc;
use uuid::Uuid;

use crate::domain::order::Order;
use crate::domain::product::Product;
use crate::domain::value_objects::{
    OrderId, OrderStatus, Price, ProductDescription, ProductId, ProductName, Quantity, ShippingFee,
    Subtotal, TaxAmount, TotalPrice,
};

pub fn sample_product() -> Product {
    sample_product_with(
        ProductId(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()),
        "Test Product",
        1000,
        10,
    )
}

pub fn sample_product_with(id: ProductId, name: &str, price: i64, stock: i32) -> Product {
    let now = Utc::now();
    Product {
        id,
        name: ProductName::new(name).unwrap(),
        price: Price::new(price).unwrap(),
        stock: Quantity::new(stock).unwrap(),
        description: ProductDescription::from("test description".to_string()),
        created_at: now,
        updated_at: now,
    }
}

pub fn sample_order(product_id: ProductId) -> Order {
    let now = Utc::now();
    Order {
        id: OrderId(Uuid::parse_str("00000000-0000-0000-0000-000000000010").unwrap()),
        product_id,
        quantity: Quantity::new(2).unwrap(),
        subtotal: Subtotal::new(2000).unwrap(),
        tax_amount: TaxAmount::new(200).unwrap(),
        shipping_fee: ShippingFee::new(500).unwrap(),
        total_price: TotalPrice::new(2700).unwrap(),
        status: OrderStatus::pending(),
        created_at: now,
    }
}
