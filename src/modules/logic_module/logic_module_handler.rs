use std::{
    thread::JoinHandle,
};
use super::{
    events::LogicCommands
};

pub struct LogicModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub logic_commands: LogicCommands, 
}
