use std::{ 
    thread::JoinHandle,
};
use crate::{
    aliases::{
        LogicEvents,
    },
};

pub struct LogicModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub sender: LogicEvents, 
}
