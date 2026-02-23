mod event_loop;
pub mod events;
mod logic_core;
pub mod logic_module_handler;
mod project;
mod project_manager;

use std::{
    thread,
    sync::Arc,
};
use calloop::{
    LoopSignal,
    channel::{
        channel, 
    },
    EventLoop,
};
use crate::{ 
    modules::{
        app_dirs::ApplicationDirectories,
        db_module::DBModuleHandler,   
    }, 
};
use self::{
    event_loop::init_event_loop,
    logic_core::LogicCore,   
    events::{
        LogicCommand, EventSender
    },
    logic_module_handler::LogicModuleHandler,
    project_manager::ProjectManager,
};


struct EventLoopResource<S> 
where 
    S: EventSender + Send + 'static
{
    logic_core: LogicCore,
    db_module_handler: DBModuleHandler,
    event_sender: S,
    loop_signal: LoopSignal,
    project_manager: ProjectManager, 
}

pub struct LogicModule;

impl LogicModule{
    pub fn init_logic_module<S>(
        event_sender: S,
        app_dirs: Arc<ApplicationDirectories>,
        db_module_handler: DBModuleHandler,
    ) -> LogicModuleHandler 
    where 
        S: EventSender + Send + 'static 
    {
        let (sender, channel) = channel::<LogicCommand>();

        let handle = thread::spawn(move||{
            let mut event_loop: EventLoop<'_, EventLoopResource<S>> = init_event_loop(channel);
            let loop_signal = event_loop.get_signal();

            let logic_core = LogicCore::new();
            let project_manager = ProjectManager::new(app_dirs);

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core, 
                db_module_handler: db_module_handler,
                event_sender: event_sender,
                loop_signal: loop_signal,
                project_manager: project_manager,
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
