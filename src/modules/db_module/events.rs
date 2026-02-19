use std::{
    path::{
        PathBuf,
    },
};

pub enum DBEvent {
    Shutdown,
    OpenConnection(PathBuf),
}
