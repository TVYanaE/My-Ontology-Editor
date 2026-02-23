use thiserror::{
    Error,
};

#[derive(Debug, Error)]
pub enum ProjectManagerError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Toml create Error: {0}")]
    TomlError(#[from] toml::ser::Error),
}
