use crate::app::service::Service;
use crate::{
    app::app_state::AppState,
    model::category::Category,
    util::pagination::{Limit, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn categories_index(
    State(app_state): State<AppState>,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Pagination { limit, offset } = pagination.unwrap_or_default().0;
    let limit = limit.unwrap_or(Limit(10));
    let offset = offset.unwrap_or_default();
    let list = app_state
        .category_service()
        .list(limit, offset)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(list))
}

pub async fn categories_create(
    State(app_state): State<AppState>,
    Json(entity): Json<Category>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .category_service()
        .create(entity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn categories_read(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .category_service()
        .read(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn categories_update(
    State(app_state): State<AppState>,
    Json(entity): Json<Category>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .category_service()
        .update(entity)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}

pub async fn categories_delete(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let entity = app_state
        .category_service()
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}
