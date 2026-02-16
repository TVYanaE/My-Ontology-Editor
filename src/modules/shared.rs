use std::{ 
    thread::JoinHandle,
};
use crate::{
    aliases::{
        LogicEvents,
    },
};

pub struct LogicModuleDescriptor {
    pub thread_handle: Option<JoinHandle<()>>,
    pub sender: LogicEvents, 
}
