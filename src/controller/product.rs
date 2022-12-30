use crate::app::error::Error;
use crate::app::service::Service;
use crate::{
    app::app_state::AppState,
    model::product::Product,
    util::pagination::{Limit, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn products_index(
    State(app_state): State<AppState>,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, Error> {
    let Pagination { limit, offset } = pagination.unwrap_or_default().0;
    let limit = limit.unwrap_or(Limit(10));
    let offset = offset.unwrap_or_default();
    let list = app_state.product_service().list(limit, offset).await?;
    Ok(Json(list))
}

pub async fn products_create(
    State(app_state): State<AppState>,
    Json(entity): Json<Product>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.product_service().create(entity).await?;
    Ok(Json(entity))
}

pub async fn products_read(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.product_service().read(id).await?;
    Ok(Json(entity))
}

pub async fn products_update(
    State(app_state): State<AppState>,
    Json(entity): Json<Product>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.product_service().update(entity).await?;
    Ok(Json(entity))
}

pub async fn products_delete(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.product_service().delete(id).await?;
    Ok(Json(entity))
}
