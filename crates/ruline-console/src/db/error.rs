use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,
}
