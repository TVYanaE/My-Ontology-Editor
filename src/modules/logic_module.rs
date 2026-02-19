mod event_loop;
mod events;
mod logic_core;

use std::{
    thread,
    sync::{Arc, RwLock},
};
use calloop::{
    LoopSignal,
    channel::{
        channel, 
    },
};
use crate::{
    aliases::{
        LogicEvents, CustomEvents,
    },
    modules::{
        app_dirs::ApplicationDirectories,
        shared::{
            db_module_handler::DBModuleHandler,
            logic_module_handler::LogicModuleHandler,
            project_manager::ProjectManager,
        },
        graphics_module::{ExternalEvent},
    }, 
};
use self::{
    event_loop::init_event_loop,
    logic_core::LogicCore,   
};
pub use self::{
    events::LogicEvent,
};


struct EventLoopResource {
    logic_core: LogicCore,
    custom_events: CustomEvents,
    logic_events: LogicEvents,
    project_manager: Arc<RwLock<ProjectManager>>,
    app_dirs: Arc<ApplicationDirectories>, 
    loop_signal: LoopSignal,
    db_module_handler: DBModuleHandler,
}

pub struct LogicModule;

impl LogicModule {
    pub fn init_logic_module(
        custom_events: CustomEvents,
        app_dirs: Arc<ApplicationDirectories>,
        project_manager: Arc<RwLock<ProjectManager>>,
        db_module_handler: DBModuleHandler,
    ) -> LogicModuleHandler {
        let (sender, channel) = channel::<LogicEvent>();

        let cloned_sender = sender.clone();

        let handle = thread::spawn(move||{
            let mut event_loop = init_event_loop(channel);
            let loop_signal = event_loop.get_signal();

            let logic_core = LogicCore::new();

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core,
                custom_events: custom_events,
                logic_events: cloned_sender,
                project_manager: project_manager,
                app_dirs: app_dirs,
                loop_signal: loop_signal,
                db_module_handler: db_module_handler,
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
            sender: sender,
        }
    }
}
