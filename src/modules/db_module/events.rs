use std::{
    path::{
        PathBuf,
    },
};
use oneshot::{
    Sender,
};
use super::{
    db_core::ProjectDBError,
};

pub enum DBEvent {
    Shutdown,
    OpenConnection{
        project_root_path: PathBuf,
        response_target: Sender<Result<(), ProjectDBError>>
    },
}
