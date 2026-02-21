use std::{
    thread::JoinHandle,
};
use crate::{
    modules::db_module::DBEvents,
};

pub struct DBModuleHandler {
    pub thread_handle: Option<JoinHandle<()>>,
    pub db_events: DBEvents,
}
