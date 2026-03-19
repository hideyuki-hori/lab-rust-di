use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use entrait::Impl;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::service::product_service_impl::ProductService;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: i64,
    pub stock: i32,
    #[serde(default)]
    pub description: String,
}

pub async fn list_products(
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = app.find_all().await?;
    Ok(Json(products))
}

pub async fn get_product(
    Path(id): Path<ProductId>,
    State(app): State<Arc<Impl<AppState>>>,
) -> Result<Json<Product>, AppError> {
    let product = app.find_by_id(id).await?;
    Ok(Json(product))
}

pub async fn create_product(
    State(app): State<Arc<Impl<AppState>>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    let name = ProductName::new(req.name).map_err(AppError::Conflict)?;
    let price = Price::new(req.price).map_err(AppError::Conflict)?;
    let product = app
        .create(
            name,
            price,
            Quantity(req.stock),
            ProductDescription::from(req.description),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(product)))
}
