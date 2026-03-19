use redis::aio::ConnectionManager;
use sqlx::PgPool;

pub struct AppState {
    pub db_pool: PgPool,
    pub redis_conn: ConnectionManager,
    pub nats_client: async_nats::Client,
}
