use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("database error: `{0}`")]
    Database(#[from] sqlx::error::Error),
    #[error("validation error: `{0}`")]
    Validation(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e),
        }
        .into_response()
    }
}
