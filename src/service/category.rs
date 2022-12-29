use crate::{
    core::{app_state::AppState, repository::Repository},
    entity::category::Category,
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
        .db_category_repo()
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
        .db_category_repo()
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
        .db_category_repo()
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
        .db_category_repo()
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
        .db_category_repo()
        .delete(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(entity))
}
