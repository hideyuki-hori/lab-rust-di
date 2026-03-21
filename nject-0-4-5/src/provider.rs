use nject::{injectable, module, provider};
use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::infrastructure::health_checker::HealthChecker;
use crate::infrastructure::nats_event_publisher::NatsEventPublisher;
use crate::infrastructure::postgres_audit_log_repository::PostgresAuditLogRepository;
use crate::infrastructure::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::redis_master_data_repository::RedisMasterDataRepository;
use crate::infrastructure::redis_product_cache::RedisProductCache;
use crate::interface::audit_log_service::AuditLogService;
use crate::interface::health_service::HealthService;
use crate::interface::master_data_service::MasterDataService;
use crate::interface::order_service::OrderService;
use crate::interface::product_service::ProductService;
use crate::service::audit_log_service_impl::AuditLogServiceImpl;
use crate::service::health_service_impl::HealthServiceImpl;
use crate::service::master_data_service_impl::MasterDataServiceImpl;
use crate::service::order_service_impl::OrderServiceImpl;
use crate::service::product_service_impl::ProductServiceImpl;

#[injectable]
#[module]
pub(crate) struct InfraModule {
    #[export(dyn crate::interface::product_repository::ProductRepository)]
    product_repository: PostgresProductRepository,

    #[export(dyn crate::interface::product_cache::ProductCache)]
    product_cache: RedisProductCache,

    #[export(dyn crate::interface::order_repository::OrderRepository)]
    order_repository: PostgresOrderRepository,

    #[export(dyn crate::interface::audit_log_repository::AuditLogRepository)]
    audit_log_repository: PostgresAuditLogRepository,

    #[export(dyn crate::interface::master_data_repository::MasterDataRepository)]
    master_data_repository: RedisMasterDataRepository,

    #[export(dyn crate::interface::event_publisher::EventPublisher)]
    event_publisher: NatsEventPublisher,

    #[export]
    health_checker: HealthChecker,
}

#[provider]
pub(crate) struct AppProvider {
    #[provide]
    pub(crate) db_pool: PgPool,

    #[provide]
    pub(crate) redis_connection: ConnectionManager,

    #[provide]
    pub(crate) nats_client: async_nats::Client,

    #[import]
    pub(crate) infra: InfraModule,
}

impl AppProvider {
    pub(crate) fn new(
        db_pool: PgPool,
        redis_connection: ConnectionManager,
        nats_client: async_nats::Client,
    ) -> Self {
        let infra: InfraModule = {
            let bootstrap = InternalBootstrap {
                db_pool: db_pool.clone(),
                redis_connection: redis_connection.clone(),
                nats_client: nats_client.clone(),
            };
            bootstrap.provide()
        };
        Self {
            db_pool,
            redis_connection,
            nats_client,
            infra,
        }
    }

    pub(crate) fn product_service(&self) -> impl ProductService + '_ {
        let service: ProductServiceImpl = self.provide();
        service
    }

    pub(crate) fn order_service(&self) -> impl OrderService + '_ {
        let service: OrderServiceImpl = self.provide();
        service
    }

    pub(crate) fn audit_log_service(&self) -> impl AuditLogService + '_ {
        let service: AuditLogServiceImpl = self.provide();
        service
    }

    pub(crate) fn master_data_service(&self) -> impl MasterDataService + '_ {
        let service: MasterDataServiceImpl = self.provide();
        service
    }

    pub(crate) fn health_service(&self) -> impl HealthService + '_ {
        let service: HealthServiceImpl = self.provide();
        service
    }
}

#[provider]
struct InternalBootstrap {
    #[provide]
    db_pool: PgPool,

    #[provide]
    redis_connection: ConnectionManager,

    #[provide]
    nats_client: async_nats::Client,
}
