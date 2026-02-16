use std::{ 
    thread::JoinHandle,
};
use calloop::{
    channel::Sender,
};
use crate::{
    modules::{
        logic_module::LogicEvent,
    },
};

pub struct LogicModuleDescriptor {
    pub thread_handle: Option<JoinHandle<()>>,
    pub sender: Sender<LogicEvent>, 
}
