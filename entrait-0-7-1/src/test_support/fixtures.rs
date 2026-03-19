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
        price: Price(price),
        stock: Quantity(stock),
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
        quantity: Quantity(2),
        subtotal: Subtotal(2000),
        tax_amount: TaxAmount(200),
        shipping_fee: ShippingFee(500),
        total_price: TotalPrice(2700),
        status: OrderStatus::pending(),
        created_at: now,
    }
}
