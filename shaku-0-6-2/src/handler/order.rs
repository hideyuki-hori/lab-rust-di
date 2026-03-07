use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use shaku_axum::Inject;
use uuid::Uuid;

use crate::domain::order::Order;
use crate::domain::value_objects::{OrderId, ProductId, Quantity};
use crate::error::AppError;
use crate::interface::order_service::OrderService;
use crate::module::AppModule;

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

pub async fn create_order(
    service: Inject<AppModule, dyn OrderService>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<(StatusCode, Json<Order>), AppError> {
    let order = service
        .create_order(ProductId(req.product_id), Quantity(req.quantity))
        .await?;
    Ok((StatusCode::CREATED, Json(order)))
}

pub async fn get_order(
    Path(id): Path<OrderId>,
    service: Inject<AppModule, dyn OrderService>,
) -> Result<Json<Order>, AppError> {
    let order = service.find_by_id(id).await?;
    Ok(Json(order))
}
