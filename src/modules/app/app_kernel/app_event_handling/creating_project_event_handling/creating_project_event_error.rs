use thiserror::Error;

use crate::modules::app::project::project_error::ProjectError;

#[derive(Debug, Error)]
pub enum CreatingProjectEventError {
    #[error("STD IO Error: {0}")]
    STDIO(#[from] std::io::Error),

    #[error("SQLX Error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Toml Crate Error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Strip Prefix Error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),  

    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),
}
