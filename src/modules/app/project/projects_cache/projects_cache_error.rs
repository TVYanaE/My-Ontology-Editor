use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectsCacheError {
    #[error("STD Error: {0}")]
    STDError(#[from] std::io::Error),

    #[error("TOML Deserialize Error: {0}")]
    TOMLDesError(#[from] toml::de::Error),

    #[error("UUID Error: {0}")]
    UuidError(#[from] uuid::Error),
}
