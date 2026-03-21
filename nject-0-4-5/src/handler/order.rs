use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::order::Order;
use crate::domain::value_objects::{OrderId, ProductId, Quantity};
use crate::error::AppError;
use crate::interface::order_service::OrderService;
use crate::provider::AppProvider;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

pub async fn create_order(
    State(prov): State<Arc<AppProvider>>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let service = prov.order_service();
    let quantity = Quantity::new(req.quantity).map_err(AppError::BadRequest)?;
    let order = service
        .create_order(ProductId(req.product_id), quantity)
        .await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_order(
    Path(id): Path<OrderId>,
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<Order>, AppError> {
    let service = prov.order_service();
    let order = service.find_by_id(id).await?;
    Ok(Json(order))
}
