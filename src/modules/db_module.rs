mod db_core;
mod db_module_handler;
mod event_loop;
mod events;

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
    db_core::DBCore,
    event_loop::init_event_loop,
};
pub use self::{
    events::{
        DBCommand, DBCommands,
        Migrations,
    },
    db_core::{
        db_core_error::DBCoreError,
    },
    db_module_handler::DBModuleHandler,
};

struct EventLoopResource {
    pub db_core: DBCore,
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

            let mut event_loop_resource = EventLoopResource {
                db_core: db_core,
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
