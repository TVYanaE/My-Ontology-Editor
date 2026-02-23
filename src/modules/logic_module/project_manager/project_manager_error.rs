use thiserror::{
    Error,
};
use crate::{
    modules::{
        db_module::{
            DBCommand, DBCoreError,
        },
    },
};

#[derive(Debug, Error)]
pub enum ProjectManagerError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Toml create Error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("MPSC Send DB Command Error: {0}")]
    MPSCSendDBCommandError(#[from] std::sync::mpsc::SendError<DBCommand>),

    #[error("DB Core Error: {0}")]
    DBCoreError(#[from] DBCoreError),

    #[error("One Shot Receive Error: {0}")]
    OneShotReceiveError(#[from] oneshot::RecvError),
}
