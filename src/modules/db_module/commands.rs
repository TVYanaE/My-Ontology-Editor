use std::{
    path::{
        PathBuf,
    },
};
use crossbeam::{
    atomic::{
        AtomicCell,
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
            db_connect_handler::DBConnectHandlerID
        }, 
    },
}; 

pub type Migrations = Vec<String>;
pub type DBCommands = Sender<DBCommand>;

#[derive(Debug)]
pub enum DBCommand {
    Shutdown,
    CreateDBFile {
        /// Full name with Data Base File Name and extension db3
        db_file_path: PathBuf,
        migrations: Option<Migrations>,
        response_target: OneShotSender<Result<DBConnectHandlerID, DBCoreError>>,
    },
    OpenDBConnect {
        db_file_path: PathBuf,
        response_target: OneShotSender<Result<DBConnectHandlerID, DBCoreError>>,
    }, 
}


