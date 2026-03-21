use async_trait::async_trait;
use nject::inject;
use sqlx::PgPool;

use crate::domain::order::Order;
use crate::domain::value_objects::OrderId;
use crate::error::AppError;
use crate::interface::order_repository::OrderRepository;

#[inject(|pool: &'prov PgPool| Self { pool: pool.clone() })]
pub(crate) struct PostgresOrderRepository {
    pub(crate) pool: PgPool,
}

#[async_trait]
impl OrderRepository for PostgresOrderRepository {
    async fn create(&self, order: &Order) -> Result<Order, AppError> {
        let created = sqlx::query_as::<_, Order>(include_str!("sql/orders/create.sql"))
            .bind(order.id)
            .bind(order.product_id)
            .bind(order.quantity)
            .bind(order.subtotal)
            .bind(order.tax_amount)
            .bind(order.shipping_fee)
            .bind(order.total_price)
            .bind(&order.status)
            .bind(order.created_at)
            .fetch_one(&self.pool)
            .await?;
        Ok(created)
    }

    async fn find_by_id(&self, id: OrderId) -> Result<Option<Order>, AppError> {
        let order = sqlx::query_as::<_, Order>(include_str!("sql/orders/find_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(order)
    }
}

#[cfg(test)]
mod integration_tests {
    use chrono::Utc;

    use crate::domain::order::Order;
    use crate::domain::product::Product;
    use crate::domain::value_objects::*;
    use crate::infrastructure::postgres_product_repository::PostgresProductRepository;
    use crate::interface::order_repository::OrderRepository;
    use crate::interface::product_repository::ProductRepository;
    use crate::test_support::test_db::TestDb;

    use super::PostgresOrderRepository;

    fn sample_product() -> Product {
        let now = Utc::now();
        Product {
            id: ProductId::new(),
            name: ProductName::new("Order Test Product").unwrap(),
            price: Price::new(1000).unwrap(),
            stock: Quantity::new(50).unwrap(),
            description: ProductDescription::from("".to_string()),
            created_at: now,
            updated_at: now,
        }
    }

    fn sample_order(product_id: ProductId) -> Order {
        let now = Utc::now();
        Order {
            id: OrderId::new(),
            product_id,
            quantity: Quantity::new(3).unwrap(),
            subtotal: Subtotal::new(3000).unwrap(),
            tax_amount: TaxAmount::new(300).unwrap(),
            shipping_fee: ShippingFee::new(500).unwrap(),
            total_price: TotalPrice::new(3800).unwrap(),
            status: OrderStatus::pending(),
            created_at: now,
        }
    }

    #[tokio::test]
    async fn create_and_find_order() {
        let db = TestDb::new().await;
        let product_repository = PostgresProductRepository {
            pool: db.pool.clone(),
        };
        let order_repository = PostgresOrderRepository {
            pool: db.pool.clone(),
        };

        let product = sample_product();
        product_repository.create(&product).await.unwrap();

        let order = sample_order(product.id);
        let created = order_repository.create(&order).await.unwrap();
        assert_eq!(created.id, order.id);
        assert_eq!(created.product_id, product.id);
        assert_eq!(created.quantity, Quantity::new(3).unwrap());
        assert_eq!(created.total_price, TotalPrice::new(3800).unwrap());

        let found = order_repository
            .find_by_id(order.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(found.id, order.id);
        assert_eq!(found.status, OrderStatus::pending());
    }

    #[tokio::test]
    async fn find_order_not_found() {
        let db = TestDb::new().await;
        let order_repository = PostgresOrderRepository {
            pool: db.pool.clone(),
        };

        let result = order_repository.find_by_id(OrderId::new()).await.unwrap();
        assert!(result.is_none());
    }
}
