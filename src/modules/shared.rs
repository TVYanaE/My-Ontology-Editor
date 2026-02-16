use std::{ 
    thread::JoinHandle,
};
use calloop::{
    channel::Sender,
};
use crate::{
    modules::{
        logic_module::events::LogicEvent,
    },
};

pub struct LogicThreadDescriptor {
    pub thread_handle: Option<JoinHandle<()>>,
    pub sender: Sender<LogicEvent>, 
}
