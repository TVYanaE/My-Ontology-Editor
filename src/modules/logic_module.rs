mod event_loop;
pub mod events;
mod logic_core;
pub mod logic_module_handler;
//mod project;

use std::{
    thread,
    sync::Arc,
};
use calloop::{
    LoopSignal,
    channel::{
        channel, Channel, 
    },
    EventLoop,
};
use crate::{ 
    modules::{
        app_dirs::ApplicationDirectories,
        
    }, 
};
use self::{
    event_loop::init_event_loop,
    logic_core::LogicCore,   
    events::{
        LogicCommand, EventSender
    },
    logic_module_handler::LogicModuleHandler,
};


struct EventLoopResource<S> 
where 
    S: EventSender + Send + 'static
{
    logic_core: LogicCore,
    event_sender: S,
    app_dirs: Arc<ApplicationDirectories>, 
    loop_signal: LoopSignal,
}

pub struct LogicModule;

impl LogicModule{
    pub fn init_logic_module<S>(
        event_sender: S,
        app_dirs: Arc<ApplicationDirectories>,
    ) -> LogicModuleHandler 
    where 
        S: EventSender + Send + 'static 
    {
        let (sender, channel) = channel::<LogicCommand>();

        let handle = thread::spawn(move||{
            let mut event_loop: EventLoop<'_, EventLoopResource<S>> = init_event_loop(channel);
            let loop_signal = event_loop.get_signal();

            let logic_core = LogicCore::new();

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core,
                event_sender: event_sender,
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
        
        LogicModuleHandler {
            thread_handle: Some(handle),
            logic_commands: sender,
        }
    }
}
