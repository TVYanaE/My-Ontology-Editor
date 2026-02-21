use std::{
    path::{
        PathBuf,
    },
};
use calloop::{
    channel::Sender,
};
use super::{
    db_core::ProjectDBError,
};

pub type DBEvents = Sender<DBEvent>;

pub enum DBEvent {
    Shutdown,
    OpenConnection{
        project_root_path: PathBuf,
        //response_target: Sender<Result<(), ProjectDBError>>
    },
}
