use thiserror::Error;

#[derive(Error, Debug)]
pub enum SqlantError {
    #[error("Postgres error {0}")]
    Postgres(#[from] tokio_postgres::Error),
    #[error("TlsConnector error {0}")]
    TlsConnector(#[from] native_tls::Error),
    #[error("Template error {0}")]
    Template(#[from] tinytemplate::error::Error),
    #[error("PsqlErdLoader error {0}")]
    PsqlErdLoader(String),
    #[error("Generator error {0}")]
    Generator(String),
}
