use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlantError {
    #[error("Postgres error {0}")]
    Postgres(#[from] postgres::Error),
}
