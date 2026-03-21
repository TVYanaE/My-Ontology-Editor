use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("STD IO Error: {0}")]
    STDIO(#[from] std::io::Error), 

    #[error("Toml Crate Error: {0}")]
    TomlDeser(#[from] toml::de::Error),

    #[error("SQLX Error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Uuid Error: {0}")]
    UuidError(#[from] uuid::Error),
}
