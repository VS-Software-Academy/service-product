use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("database error: `{0}`")]
    Database(#[from] sqlx::error::Error),
    #[error("validation error: `{0}`")]
    Validation(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::Validation(e) => (StatusCode::BAD_REQUEST, e),
        }
        .into_response()
    }
}
