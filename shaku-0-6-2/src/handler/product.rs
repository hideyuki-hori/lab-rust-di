use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use shaku_axum::Inject;

use crate::domain::product::Product;
use crate::domain::value_objects::{Price, ProductDescription, ProductId, ProductName, Quantity};
use crate::error::AppError;
use crate::interface::product_service::ProductService;
use crate::module::AppModule;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: i64,
    pub stock: i32,
    #[serde(default)]
    pub description: String,
}

pub async fn list_products(
    service: Inject<AppModule, dyn ProductService>,
) -> Result<Json<Vec<Product>>, AppError> {
    let products = service.find_all().await?;
    Ok(Json(products))
}

pub async fn get_product(
    Path(id): Path<ProductId>,
    service: Inject<AppModule, dyn ProductService>,
) -> Result<Json<Product>, AppError> {
    let product = service.find_by_id(id).await?;
    Ok(Json(product))
}

pub async fn create_product(
    service: Inject<AppModule, dyn ProductService>,
    Json(req): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    let name = ProductName::new(req.name).map_err(AppError::Conflict)?;
    let price = Price::new(req.price).map_err(AppError::Conflict)?;
    let product = service
        .create(
            name,
            price,
            Quantity(req.stock),
            ProductDescription::from(req.description),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(product)))
}
