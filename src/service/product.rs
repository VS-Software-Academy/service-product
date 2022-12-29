use crate::{
    core::{app_state::AppState, repository::Repository},
    entity::product::Product,
    util::pagination::{Limit, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn products_index(
    State(app_state): State<AppState>,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Pagination { limit, offset } = pagination.unwrap_or_default().0;
    let limit = limit.unwrap_or(Limit(10));
    let offset = offset.unwrap_or_default();
    let list = app_state
        .db_product_repo()
        .list(limit, offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(list))
}

pub async fn products_create(
    State(app_state): State<AppState>,
    Json(entity): Json<Product>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .db_product_repo()
        .create(entity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn products_read(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .db_product_repo()
        .read(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn products_update(
    State(app_state): State<AppState>,
    Json(entity): Json<Product>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .db_product_repo()
        .update(entity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn products_delete(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .db_product_repo()
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}
