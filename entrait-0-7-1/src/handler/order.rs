use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use entrait::Impl;
use serde::Deserialize;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::domain::order::Order;
use crate::domain::value_objects::{OrderId, ProductId, Quantity};
use crate::error::AppError;
use crate::service::order_service_impl::OrderService;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

pub async fn create_order(
    State(app): State<Arc<Impl<AppState>>>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let order = app
        .create_order(ProductId(req.product_id), Quantity(req.quantity))
        .await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_order(
    Path(id): Path<OrderId>,
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<Order>, AppError> {
    let order = app.find_order_by_id_svc(id).await?;
    Ok(Json(order))
}
