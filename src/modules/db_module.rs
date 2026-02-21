mod db_core;
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
use crate::{ 
    modules::{
        shared::{
            db_module_handler::DBModuleHandler,
        },
    },
};
use self::{
    db_core::DBCore,
    event_loop::init_event_loop,
};
pub use self::{
    events::{DBEvent, DBEvents},
    db_core::ProjectDBError,
};

struct EventLoopResource {
    pub db_core: DBCore,
    pub db_events: DBEvents, 
    pub loop_signal: LoopSignal,
}

pub struct DBModule; 

impl DBModule {
    pub fn init_db_module() -> DBModuleHandler {
        let (db_events, channel) = channel::<DBEvent>(); 

        let db_events_cloned = db_events.clone(); 

        let thread_handle = thread::spawn(||{
            let mut event_loop = init_event_loop(channel);
            let loop_signal = event_loop.get_signal();

            let db_core = DBCore::new();

            let mut event_loop_resource = EventLoopResource {
                db_core: db_core,
                db_events: db_events_cloned,
                loop_signal: loop_signal, 
            };

            let _ = event_loop.run(None, &mut event_loop_resource, |event_loop_resource|{
                if event_loop_resource.db_core.is_shutdown() {
                    event_loop_resource.loop_signal.stop(); 
                } 
            });
        });

        DBModuleHandler {
            db_events: db_events,
            thread_handle: Some(thread_handle), 
        }
    } 
}
