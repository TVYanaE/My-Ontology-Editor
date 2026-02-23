use std::{
    thread::JoinHandle
};
use crate::{
    modules::{
        db_module::{
            events::{
                DBCommands
            },
        },
    },
};

pub struct DBModuleHandler {
    pub db_commands: DBCommands,
    pub thread_handle: Option<JoinHandle<()>>, 
} 
