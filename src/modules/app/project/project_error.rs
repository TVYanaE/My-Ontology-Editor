use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error), 

    #[error("Toml Crate Error: {0}")]
    TomlDeserError(#[from] toml::de::Error),

    #[error("SQLX Error: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[error("Uuid Error: {0}")]
    UuidError(#[from] uuid::Error),
}
