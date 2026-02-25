mod commands;
mod db_connect_cache;
mod db_connect_handler;
mod db_core;
mod db_module_handler;
mod event_loop;

use std::{
    thread,
};
use calloop::{
    channel::{
        channel,
    },
    LoopSignal,
};

use self::{
    db_connect_cache::{
        DBConnectCache,
    },
    db_core::DBCore,
    event_loop::init_event_loop,
};
pub use self::{
    commands::{
        DBCommand, DBCommands,
        Migrations,
    },
    db_core::{
        db_core_error::DBCoreError,
    },
    db_module_handler::{
        DBModuleHandler,
    },
    db_connect_handler::{
        DBConnectHandlerID
    },
};

struct EventLoopResource {
    pub db_core: DBCore,
    pub db_connect_cache: DBConnectCache,
    pub loop_signal: LoopSignal,
}

pub struct DBModule; 

impl DBModule {
    pub fn init_db_module() -> DBModuleHandler {
        let (db_commands, channel) = channel::<DBCommand>(); 

        let thread_handle = thread::spawn(||{
            let mut event_loop = init_event_loop(channel);
            let loop_signal = event_loop.get_signal();

            let db_core = DBCore::new();
            let db_connect_cache = DBConnectCache::new();

            let mut event_loop_resource = EventLoopResource {
                db_core: db_core,
                db_connect_cache: db_connect_cache,
                loop_signal: loop_signal, 
            };

            let _ = event_loop.run(None, &mut event_loop_resource, |event_loop_resource|{
                if event_loop_resource.db_core.is_shutdown() { 
                    println!("DB module shutdown");
                    event_loop_resource.loop_signal.stop(); 
                } 
            });
        });

        DBModuleHandler {
            db_commands: db_commands,
            thread_handle: Some(thread_handle),
        } 
    } 
}
