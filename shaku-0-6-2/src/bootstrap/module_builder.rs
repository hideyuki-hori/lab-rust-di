use std::sync::Arc;

use crate::infrastructure;
use crate::infrastructure::health_checker::HealthCheckerParameters;
use crate::infrastructure::postgres_audit_log_repository::PostgresAuditLogRepositoryParameters;
use crate::infrastructure::nats_event_publisher::NatsEventPublisherParameters;
use crate::infrastructure::postgres_order_repository::PostgresOrderRepositoryParameters;
use crate::infrastructure::postgres_product_repository::PostgresProductRepositoryParameters;
use crate::infrastructure::redis_master_data_repository::RedisMasterDataRepositoryParameters;
use crate::infrastructure::redis_product_cache::RedisProductCacheParameters;
use crate::module::{AppModule, InfraModule};

use super::connections::Connections;

pub struct Modules {
    pub app: Arc<AppModule>,
    pub infra: Arc<InfraModule>,
}

pub fn build(connections: Connections) -> Modules {
    let infra = Arc::new(
        InfraModule::builder()
            .with_component_parameters::<infrastructure::postgres_product_repository::PostgresProductRepository>(
                PostgresProductRepositoryParameters {
                    pool: connections.db_pool.clone(),
                },
            )
            .with_component_parameters::<infrastructure::postgres_order_repository::PostgresOrderRepository>(
                PostgresOrderRepositoryParameters {
                    pool: connections.db_pool.clone(),
                },
            )
            .with_component_parameters::<infrastructure::redis_product_cache::RedisProductCache>(
                RedisProductCacheParameters {
                    conn: connections.redis_conn.clone(),
                },
            )
            .with_component_parameters::<infrastructure::redis_master_data_repository::RedisMasterDataRepository>(
                RedisMasterDataRepositoryParameters {
                    conn: connections.redis_conn.clone(),
                },
            )
            .with_component_parameters::<infrastructure::nats_event_publisher::NatsEventPublisher>(
                NatsEventPublisherParameters {
                    client: connections.nats_client.clone(),
                },
            )
            .with_component_parameters::<infrastructure::health_checker::HealthChecker>(
                HealthCheckerParameters {
                    pool: connections.db_pool.clone(),
                    redis_conn: connections.redis_conn,
                    nats_client: connections.nats_client,
                },
            )
            .with_component_parameters::<infrastructure::postgres_audit_log_repository::PostgresAuditLogRepository>(
                PostgresAuditLogRepositoryParameters {
                    pool: connections.db_pool,
                },
            )
            .build(),
    );

    let app = Arc::new(AppModule::builder(infra.clone()).build());

    Modules { app, infra }
}
