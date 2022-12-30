use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("database error: `{0}`")]
    Database(#[from] sqlx::error::Error),
    #[error("validation error: `{0}`")]
    Validation(String),
}
