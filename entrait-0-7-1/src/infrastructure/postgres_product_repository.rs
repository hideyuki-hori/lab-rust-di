use async_trait::async_trait;
use entrait::Impl;

use crate::app_state::AppState;
use crate::domain::product::Product;
use crate::domain::value_objects::{ProductId, Quantity};
use crate::error::AppError;
use crate::interface::product_repository::ProductRepository;

#[async_trait]
impl ProductRepository for Impl<AppState> {
    async fn find_all_products(&self) -> Result<Vec<Product>, AppError> {
        let products = sqlx::query_as::<_, Product>(include_str!("sql/products/find_all.sql"))
            .fetch_all(&self.db_pool)
            .await?;
        Ok(products)
    }

    async fn find_product_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError> {
        let product = sqlx::query_as::<_, Product>(include_str!("sql/products/find_by_id.sql"))
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await?;
        Ok(product)
    }

    async fn create_product(&self, product: &Product) -> Result<Product, AppError> {
        let created = sqlx::query_as::<_, Product>(include_str!("sql/products/create.sql"))
            .bind(product.id)
            .bind(&product.name)
            .bind(product.price)
            .bind(product.stock)
            .bind(&product.description)
            .bind(product.created_at)
            .bind(product.updated_at)
            .fetch_one(&self.db_pool)
            .await?;
        Ok(created)
    }

    async fn update_product_stock(&self, id: ProductId, delta: Quantity) -> Result<(), AppError> {
        let result = sqlx::query(include_str!("sql/products/update_stock.sql"))
            .bind(delta)
            .bind(id)
            .execute(&self.db_pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Product {id} not found")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::domain::value_objects::{ProductId, Quantity};
    use crate::interface::product_repository::ProductRepository;
    use crate::test_support::fixtures::{sample_product, sample_product_with};
    use crate::test_support::test_db::TestDb;

    #[tokio::test]
    async fn create_and_find_by_id() {
        let db = TestDb::new().await;
        let product = sample_product();

        let created = db.app.create_product(&product).await.unwrap();
        assert_eq!(created.id, product.id);
        assert_eq!(created.name.to_string(), "Test Product");
        assert_eq!(created.stock, Quantity(10));

        let found = db.app.find_product_by_id(product.id).await.unwrap().unwrap();
        assert_eq!(found.id, product.id);
        assert_eq!(found.name.to_string(), "Test Product");
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let db = TestDb::new().await;

        let result = db.app.find_product_by_id(ProductId::new()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_all_returns_products() {
        let db = TestDb::new().await;

        let p1 = sample_product();
        let p2 = sample_product_with(
            ProductId::new(),
            "Second Product",
            2000,
            5,
        );

        db.app.create_product(&p1).await.unwrap();
        db.app.create_product(&p2).await.unwrap();

        let all = db.app.find_all_products().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn update_stock_decreases() {
        let db = TestDb::new().await;
        let product = sample_product();
        db.app.create_product(&product).await.unwrap();

        db.app.update_product_stock(product.id, Quantity(-3)).await.unwrap();

        let updated = db.app.find_product_by_id(product.id).await.unwrap().unwrap();
        assert_eq!(updated.stock, Quantity(7));
    }

    #[tokio::test]
    async fn update_stock_not_found() {
        let db = TestDb::new().await;

        let result = db.app.update_product_stock(ProductId::new(), Quantity(-1)).await;
        assert!(result.is_err());
    }
}
