use sqlx::PgPool;
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;

pub struct TestDb {
    pub pool: PgPool,
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

        Self {
            pool,
            _container: container,
        }
    }
}
