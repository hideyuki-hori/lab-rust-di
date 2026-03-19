use async_trait::async_trait;
use entrait::Impl;
use redis::AsyncCommands;

use crate::app_state::AppState;
use crate::domain::product::Product;
use crate::domain::value_objects::ProductId;
use crate::error::AppError;
use crate::interface::product_cache::ProductCache;

const CACHE_TTL_SECS: u64 = 300;

#[async_trait]
impl ProductCache for Impl<AppState> {
    async fn cache_get_all(&self) -> Result<Option<Vec<Product>>, AppError> {
        let mut conn = self.redis_conn.clone();
        let data: Option<String> = conn.get("products:all").await?;
        match data {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    async fn cache_set_all(&self, products: &[Product]) -> Result<(), AppError> {
        let mut conn = self.redis_conn.clone();
        let json = serde_json::to_string(products)?;
        conn.set_ex::<_, _, ()>("products:all", &json, CACHE_TTL_SECS)
            .await?;
        Ok(())
    }

    async fn cache_get_by_id(&self, id: ProductId) -> Result<Option<Product>, AppError> {
        let mut conn = self.redis_conn.clone();
        let key = format!("product:{id}");
        let data: Option<String> = conn.get(&key).await?;
        match data {
            Some(json) => Ok(Some(serde_json::from_str(&json)?)),
            None => Ok(None),
        }
    }

    async fn cache_set_by_id(&self, product: &Product) -> Result<(), AppError> {
        let mut conn = self.redis_conn.clone();
        let key = format!("product:{}", product.id);
        let json = serde_json::to_string(product)?;
        conn.set_ex::<_, _, ()>(&key, &json, CACHE_TTL_SECS).await?;
        Ok(())
    }

    async fn cache_invalidate(&self) -> Result<(), AppError> {
        let mut conn = self.redis_conn.clone();
        let mut cursor: u64 = 0;
        let mut all_keys = Vec::new();
        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg("product*")
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await?;
            all_keys.extend(keys);
            if next_cursor == 0 {
                break;
            }
            cursor = next_cursor;
        }
        if !all_keys.is_empty() {
            let _: () = redis::cmd("DEL")
                .arg(&all_keys)
                .query_async(&mut conn)
                .await?;
        }
        Ok(())
    }
}
