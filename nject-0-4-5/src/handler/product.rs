use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::interface::product_service::ProductService;
use crate::provider::AppProvider;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: i64,
    pub stock: i32,
    #[serde(default)]
    pub description: String,
}

pub async fn list_products(
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<Vec<Product>>, AppError> {
    let service = prov.product_service();
    let products = service.find_all().await?;
    Ok(Json(products))
}

pub async fn get_product(
    Path(id): Path<ProductId>,
    State(prov): State<Arc<AppProvider>>,
) -> Result<Json<Product>, AppError> {
    let service = prov.product_service();
    let product = service.find_by_id(id).await?;
    Ok(Json(product))
}

pub async fn create_product(
    State(prov): State<Arc<AppProvider>>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    let service = prov.product_service();
    let name = ProductName::new(req.name).map_err(AppError::BadRequest)?;
    let price = Price::new(req.price).map_err(AppError::BadRequest)?;
    let stock = Quantity::new(req.stock).map_err(AppError::BadRequest)?;
    let product = service
        .create(
            name,
            price,
            stock,
            ProductDescription::from(req.description),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(product)))
}
