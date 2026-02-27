use std::{
    thread::JoinHandle,
};
use super::{
    logic_module_io::logic_command::LogicCommands,
};

pub struct LogicModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub logic_commands: LogicCommands, 
}
