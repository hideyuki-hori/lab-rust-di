use async_trait::async_trait;
use entrait::Impl;

use crate::app_state::AppState;
use crate::domain::order::Order;
use crate::domain::value_objects::OrderId;
use crate::error::AppError;
use crate::interface::order_repository::OrderRepository;

#[async_trait]
impl OrderRepository for Impl<AppState> {
    async fn create_order_record(&self, order: &Order) -> Result<Order, AppError> {
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
            .fetch_one(&self.db_pool)
            .await?;
        Ok(created)
    }

    async fn find_order_by_id(&self, id: OrderId) -> Result<Option<Order>, AppError> {
        let order = sqlx::query_as::<_, Order>(include_str!("sql/orders/find_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(order)
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::domain::value_objects::*;
    use crate::interface::order_repository::OrderRepository;
    use crate::interface::product_repository::ProductRepository;
    use crate::test_support::fixtures::{sample_order, sample_product};
    use crate::test_support::test_db::TestDb;

    #[tokio::test]
    async fn create_and_find_order() {
        let db = TestDb::new().await;
        let product = sample_product();
        db.app.create_product(&product).await.unwrap();

        let order = sample_order(product.id);
        let created = db.app.create_order_record(&order).await.unwrap();
        assert_eq!(created.id, order.id);
        assert_eq!(created.product_id, product.id);
        assert_eq!(created.quantity, Quantity(2));
        assert_eq!(created.total_price, TotalPrice(2700));

        let found = db.app.find_order_by_id(order.id).await.unwrap().unwrap();
        assert_eq!(found.id, order.id);
        assert_eq!(found.status, OrderStatus::pending());
    }

    #[tokio::test]
    async fn find_order_not_found() {
        let db = TestDb::new().await;

        let result = db.app.find_order_by_id(OrderId::new()).await.unwrap();
        assert!(result.is_none());
    }
}
