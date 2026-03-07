use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;

use crate::handler;
use crate::module::AppModule;

pub fn create(module: Arc<AppModule>) -> Router {
    Router::new()
        .route(
            "/products",
            get(handler::product::list_products).post(handler::product::create_product),
        )
        .route("/products/{id}", get(handler::product::get_product))
        .route("/orders", post(handler::order::create_order))
        .route("/orders/{id}", get(handler::order::get_order))
        .route("/master-data", get(handler::master_data::list_master_data))
        .route(
            "/master-data/{key}",
            get(handler::master_data::get_master_data).put(handler::master_data::set_master_data),
        )
        .route("/audit-logs", get(handler::audit_log::list_audit_logs))
        .route("/health", get(handler::health::health_check))
        .with_state(module)
}
