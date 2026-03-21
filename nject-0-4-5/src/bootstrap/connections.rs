use redis::aio::ConnectionManager;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::AppConfig;

pub struct Connections {
    pub db_pool: PgPool,
    pub redis_connection: ConnectionManager,
    pub nats_client: async_nats::Client,
}

impl Connections {
    pub async fn establish(config: &AppConfig) -> anyhow::Result<Self> {
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.database_url)
            .await?;
        tracing::info!("Connected to PostgreSQL");

        let redis_client = redis::Client::open(config.redis_url.as_str())?;
        let redis_connection = ConnectionManager::new(redis_client).await?;
        tracing::info!("Connected to Redis");

        let nats_client = async_nats::connect(&config.nats_url).await?;
        tracing::info!("Connected to NATS");

        Ok(Self {
            db_pool,
            redis_connection,
            nats_client,
        })
    }
}
