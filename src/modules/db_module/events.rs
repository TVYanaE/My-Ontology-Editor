use std::{
    path::{
        PathBuf,
    },
};
use calloop::{
    channel::Sender,
};
use crate::{
    aliases::{
        OneShotSender,
    },
    modules::{
        db_module::{
            db_core::{
                db_core_error::DBCoreError,
            },
        },
    },
}; 

pub type Migrations = Vec<String>;
pub type DBCommands = Sender<DBCommand>;

pub enum DBCommand {
    Shutdown,
    CreateDBFile {
        /// Full name with Data Base File Name and extension db3
        db_file_path: PathBuf,
        migrations: Option<Migrations>,
        response_target: OneShotSender<Result<(), DBCoreError>>
    },
}
