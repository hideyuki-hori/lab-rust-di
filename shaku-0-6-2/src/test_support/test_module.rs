use std::sync::Arc;

use crate::interface::audit_log_repository::MockAuditLogRepository;
use crate::interface::event_publisher::MockEventPublisher;
use crate::interface::health_service::MockHealthService;
use crate::interface::master_data_repository::MockMasterDataRepository;
use crate::interface::order_repository::MockOrderRepository;
use crate::interface::product_cache::MockProductCache;
use crate::interface::product_repository::MockProductRepository;
use crate::module::{AppModule, InfraModule};

use crate::interface::audit_log_repository::AuditLogRepository;
use crate::interface::event_publisher::EventPublisher;
use crate::interface::health_service::HealthService;
use crate::interface::master_data_repository::MasterDataRepository;
use crate::interface::order_repository::OrderRepository;
use crate::interface::product_cache::ProductCache;
use crate::interface::product_repository::ProductRepository;

pub struct TestModuleBuilder {
    product_repository: MockProductRepository,
    product_cache: MockProductCache,
    order_repository: MockOrderRepository,
    event_publisher: MockEventPublisher,
    master_data: MockMasterDataRepository,
}

impl TestModuleBuilder {
    pub fn new() -> Self {
        Self {
            product_repository: MockProductRepository::new(),
            product_cache: MockProductCache::new(),
            order_repository: MockOrderRepository::new(),
            event_publisher: MockEventPublisher::new(),
            master_data: MockMasterDataRepository::new(),
        }
    }

    pub fn with_product_repository(mut self, repository: MockProductRepository) -> Self {
        self.product_repository = repository;
        self
    }

    pub fn with_product_cache(mut self, cache: MockProductCache) -> Self {
        self.product_cache = cache;
        self
    }

    pub fn with_order_repository(mut self, repository: MockOrderRepository) -> Self {
        self.order_repository = repository;
        self
    }

    pub fn with_event_publisher(mut self, publisher: MockEventPublisher) -> Self {
        self.event_publisher = publisher;
        self
    }

    pub fn with_master_data(mut self, master_data: MockMasterDataRepository) -> Self {
        self.master_data = master_data;
        self
    }

    pub fn build(self) -> Arc<AppModule> {
        let infra_module = Arc::new(
            InfraModule::builder()
                .with_component_override::<dyn ProductRepository>(Box::new(self.product_repository))
                .with_component_override::<dyn ProductCache>(Box::new(self.product_cache))
                .with_component_override::<dyn OrderRepository>(Box::new(self.order_repository))
                .with_component_override::<dyn EventPublisher>(Box::new(self.event_publisher))
                .with_component_override::<dyn MasterDataRepository>(Box::new(self.master_data))
                .with_component_override::<dyn HealthService>(Box::new(MockHealthService::new()))
                .with_component_override::<dyn AuditLogRepository>(Box::new(MockAuditLogRepository::new()))
                .build(),
        );
        Arc::new(AppModule::builder(infra_module).build())
    }
}
