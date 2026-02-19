use std::{
    thread::JoinHandle,
};
use crate::{
    aliases::DBEvents, 
};

pub struct DBModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub db_events: DBEvents,
}
