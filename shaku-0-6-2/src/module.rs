use shaku::module;

use crate::infrastructure::health_checker::HealthChecker;
use crate::infrastructure::postgres_audit_log_repository::PostgresAuditLogRepository;
use crate::infrastructure::nats_event_publisher::NatsEventPublisher;
use crate::infrastructure::postgres_order_repository::PostgresOrderRepository;
use crate::infrastructure::postgres_product_repository::PostgresProductRepository;
use crate::infrastructure::redis_master_data_repository::RedisMasterDataRepository;
use crate::infrastructure::redis_product_cache::RedisProductCache;
use crate::service::order_service_impl::OrderServiceImpl;
use crate::service::product_service_impl::ProductServiceImpl;

module! {
    pub InfraModule {
        components = [
            PostgresProductRepository,
            PostgresOrderRepository,
            RedisProductCache,
            RedisMasterDataRepository,
            NatsEventPublisher,
            HealthChecker,
            PostgresAuditLogRepository,
        ],
        providers = []
    }
}

module! {
    pub AppModule {
        components = [
            ProductServiceImpl,
            OrderServiceImpl,
        ],
        providers = [],
        use InfraModule {
            components = [
                dyn crate::interface::product_repository::ProductRepository,
                dyn crate::interface::product_cache::ProductCache,
                dyn crate::interface::order_repository::OrderRepository,
                dyn crate::interface::event_publisher::EventPublisher,
                dyn crate::interface::master_data_repository::MasterDataRepository,
                dyn crate::interface::health_service::HealthService,
                dyn crate::interface::audit_log_repository::AuditLogRepository,
            ],
            providers = []
        }
    }
}
