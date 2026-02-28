use thiserror::{
    Error,
};
use crate::{
    modules::{
        logic_module::{
            db_core::{
                db_core_error::DBCoreError,
            },
        },
    },
};

#[derive(Debug, Error)]
pub enum ProjectManagerError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Toml create Error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("DB Core Error: {0}")]
    DBCoreError(#[from] DBCoreError),
}
