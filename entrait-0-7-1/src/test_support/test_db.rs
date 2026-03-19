use std::sync::Arc;

use entrait::Impl;
use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

use crate::app_state::AppState;

pub struct TestDb {
    pub app: Arc<Impl<AppState>>,
    _container: ContainerAsync<Postgres>,
}

impl TestDb {
    pub async fn new() -> Self {
        let container = Postgres::default().start().await.unwrap();
        let host = container.get_host().await.unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let url = format!("postgres://postgres:postgres@{host}:{port}/postgres");

        let pool = PgPool::connect(&url).await.unwrap();

        sqlx::raw_sql(include_str!("../../migrations/001_init.sql"))
            .execute(&pool)
            .await
            .unwrap();

        let redis_conn = redis::Client::open("redis://localhost:6379")
            .unwrap()
            .get_connection_manager()
            .await
            .unwrap();
        let nats_client = async_nats::connect("nats://localhost:4222").await.unwrap();

        let app = Arc::new(Impl::new(AppState {
            db_pool: pool.clone(),
            redis_conn,
            nats_client,
        }));

        Self {
            app,
            _container: container,
        }
    }
}
