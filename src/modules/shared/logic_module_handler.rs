use std::{ 
    thread::JoinHandle,
};
use crate::{
    modules::{
        logic_module::LogicEvents,
    },
};

pub struct LogicModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub sender: LogicEvents, 
}
