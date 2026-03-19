use async_trait::async_trait;
use entrait::Impl;
use redis::AsyncCommands;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::interface::master_data_repository::MasterDataRepository;

const KEY_PREFIX: &str = "master:";

#[async_trait]
impl MasterDataRepository for Impl<AppState> {
    async fn get_master_data(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.redis_conn.clone();
        let full_key = format!("{KEY_PREFIX}{key}");
        let value: Option<String> = conn.get(&full_key).await?;
        Ok(value)
    }

    async fn set_master_data(&self, key: &str, value: &str) -> Result<(), AppError> {
        let mut conn = self.redis_conn.clone();
        let full_key = format!("{KEY_PREFIX}{key}");
        conn.set::<_, _, ()>(&full_key, value).await?;
        Ok(())
    }

    async fn get_all_master_data(&self) -> Result<Vec<(String, String)>, AppError> {
        let mut conn = self.redis_conn.clone();
        let mut cursor: u64 = 0;
        let mut keys = Vec::new();
        loop {
            let (next_cursor, batch): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(format!("{KEY_PREFIX}*"))
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await?;
            keys.extend(batch);
            if next_cursor == 0 {
                break;
            }
            cursor = next_cursor;
        }
        let mut entries = Vec::new();
        for key in keys {
            let value: Option<String> = conn.get(&key).await?;
            if let Some(v) = value {
                let short_key = match key.strip_prefix(KEY_PREFIX) {
                    Some(k) => k.to_string(),
                    None => key,
                };
                entries.push((short_key, v));
            }
        }
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(entries)
    }
}
