use async_trait::async_trait;
use shaku::Component;
use sqlx::PgPool;

use crate::domain::product::Product;
use crate::domain::value_objects::{ProductId, Quantity};
use crate::error::AppError;
use crate::interface::product_repository::ProductRepository;

#[derive(Component)]
#[shaku(interface = ProductRepository)]
pub struct PostgresProductRepository {
    pub(crate) pool: PgPool,
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, AppError> {
        let products =
            sqlx::query_as::<_, Product>(include_str!("sql/products/find_all.sql"))
                .fetch_all(&self.pool)
                .await?;
        Ok(products)
    }

    async fn find_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError> {
        let product =
            sqlx::query_as::<_, Product>(include_str!("sql/products/find_by_id.sql"))
                .bind(id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(product)
    }

    async fn create(&self, product: &Product) -> Result<Product, AppError> {
        let created =
            sqlx::query_as::<_, Product>(include_str!("sql/products/create.sql"))
                .bind(product.id)
                .bind(&product.name)
                .bind(product.price)
                .bind(product.stock)
                .bind(&product.description)
                .bind(product.created_at)
                .bind(product.updated_at)
                .fetch_one(&self.pool)
                .await?;
        Ok(created)
    }

    async fn update_stock(&self, id: ProductId, delta: Quantity) -> Result<(), AppError> {
        let result = sqlx::query(include_str!("sql/products/update_stock.sql"))
            .bind(delta)
            .bind(id)
            .execute(&self.pool)
            .await?;
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Product {id} not found")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod integration_tests {
    use chrono::Utc;

    use crate::domain::product::Product;
    use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
    use crate::interface::product_repository::ProductRepository;
    use crate::test_support::test_db::TestDb;

    use super::PostgresProductRepository;

    fn sample_product() -> Product {
        let now = Utc::now();
        Product {
            id: ProductId::new(),
            name: ProductName::new("Test Product").unwrap(),
            price: Price::new(1000).unwrap(),
            stock: Quantity(10),
            description: ProductDescription::from("A test product".to_string()),
            created_at: now,
            updated_at: now,
        }
    }

    fn build_repository(pool: sqlx::PgPool) -> PostgresProductRepository {
        PostgresProductRepository { pool }
    }

    #[tokio::test]
    async fn create_and_find_by_id() {
        let db = TestDb::new().await;
        let repository = build_repository(db.pool.clone());
        let product = sample_product();

        let created = repository.create(&product).await.unwrap();
        assert_eq!(created.id, product.id);
        assert_eq!(created.name.to_string(), "Test Product");
        assert_eq!(created.stock, Quantity(10));

        let found = repository.find_by_id(product.id).await.unwrap().unwrap();
        assert_eq!(found.id, product.id);
        assert_eq!(found.name.to_string(), "Test Product");
    }

    #[tokio::test]
    async fn find_by_id_not_found() {
        let db = TestDb::new().await;
        let repository = build_repository(db.pool.clone());

        let result = repository.find_by_id(ProductId::new()).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn find_all_returns_products() {
        let db = TestDb::new().await;
        let repository = build_repository(db.pool.clone());

        let p1 = sample_product();
        let mut p2 = sample_product();
        p2.name = ProductName::new("Second Product").unwrap();

        repository.create(&p1).await.unwrap();
        repository.create(&p2).await.unwrap();

        let all = repository.find_all().await.unwrap();
        assert_eq!(all.len(), 2);
    }

    #[tokio::test]
    async fn update_stock_decreases() {
        let db = TestDb::new().await;
        let repository = build_repository(db.pool.clone());

        let product = sample_product();
        repository.create(&product).await.unwrap();

        repository.update_stock(product.id, Quantity(-3)).await.unwrap();

        let updated = repository.find_by_id(product.id).await.unwrap().unwrap();
        assert_eq!(updated.stock, Quantity(7));
    }

    #[tokio::test]
    async fn update_stock_not_found() {
        let db = TestDb::new().await;
        let repository = build_repository(db.pool.clone());

        let result = repository.update_stock(ProductId::new(), Quantity(-1)).await;
        assert!(result.is_err());
    }
}
