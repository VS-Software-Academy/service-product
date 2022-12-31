use crate::app::error::Error;
use crate::app::service::Service;
use crate::{
    app::app_state::AppState,
    model::category::Category,
    util::pagination::{Limit, Pagination},
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

pub async fn categories_index(
    State(app_state): State<AppState>,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, Error> {
    let Pagination { limit, offset } = pagination.unwrap_or_default().0;
    let limit = limit.unwrap_or(Limit(10));
    let offset = offset.unwrap_or_default();
    let list = app_state.category_service().list(limit, offset).await?;
    Ok(Json(list))
}

pub async fn categories_create(
    State(app_state): State<AppState>,
    Json(entity): Json<Category>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.category_service().create(entity).await?;
    Ok(Json(entity))
}

pub async fn categories_read(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.category_service().read(id).await?;
    Ok(Json(entity))
}

pub async fn categories_update(
    State(app_state): State<AppState>,
    Json(entity): Json<Category>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.category_service().update(entity).await?;
    Ok(Json(entity))
}

pub async fn categories_delete(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, Error> {
    let entity = app_state.category_service().delete(id).await?;
    Ok(Json(entity))
}
