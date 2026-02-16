mod event_loop;
mod events;
mod logic_core;

use std::{
    thread,
    sync::Arc
};
use calloop::{
    LoopSignal,
    channel::{
        channel 
    },
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        graphics_module::{
            CustomEvents,
        },
        shared::LogicModuleDescriptor,
    }, 
};
use self::{
    event_loop::init_event_loop,
    logic_core::LogicCore,   
};
pub use self::{
    events::{LogicEvent, ProjectDescriptor}
};

pub struct EventLoopResource {
    logic_core: LogicCore,
    custom_events: CustomEvents,
    app_dirs: Arc<ApplicationDirectories>, 
    loop_signal: LoopSignal,
}

pub struct LogicModule;

impl LogicModule {
    pub fn init_logic_module(
        custom_events: CustomEvents,
        app_dirs: Arc<ApplicationDirectories>,
    ) -> LogicModuleDescriptor {
        let (sender, channel) = channel::<LogicEvent>();

        let handle = thread::spawn(move||{
            let mut event_loop = init_event_loop(channel)
                .expect("Event Loop Error init calloop");
            let loop_signal = event_loop.get_signal();

            let logic_core = LogicCore::new();

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core,
                custom_events: custom_events,
                app_dirs: app_dirs,
                loop_signal: loop_signal,
            }; 

            let _ = event_loop.run(None, &mut event_loop_resource, |event_loop_resource|{
                if event_loop_resource.logic_core.is_shutdown() {
                    println!("logic module shutdown");
                    event_loop_resource.loop_signal.stop();
                } 
            }); 
        });
        
        LogicModuleDescriptor {
            thread_handle: Some(handle),
            sender: sender,
        }
    }
}
