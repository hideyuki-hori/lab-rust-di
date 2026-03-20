use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use entrait::Impl;

use crate::app_state::AppState;
use crate::handler;

type App = Impl<AppState>;

pub fn create(app: Arc<Impl<AppState>>) -> Router {
    Router::new()
        .route(
            "/products",
            get(handler::product::list_products::<App>)
                .post(handler::product::create_product::<App>),
        )
        .route("/products/{id}", get(handler::product::get_product::<App>))
        .route("/orders", post(handler::order::create_order::<App>))
        .route("/orders/{id}", get(handler::order::get_order::<App>))
        .route(
            "/master-data",
            get(handler::master_data::list_master_data::<App>),
        )
        .route(
            "/master-data/{key}",
            get(handler::master_data::get_master_data::<App>)
                .put(handler::master_data::set_master_data::<App>),
        )
        .route(
            "/audit-logs",
            get(handler::audit_log::list_audit_logs::<App>),
        )
        .route("/health", get(handler::health::health_check::<App>))
        .with_state(app)
}
