mod confirmation_cache;
mod event_loop;
mod event_manager;
pub mod events;
mod job_manager;
mod logic_core;
pub mod logic_module_handler;
mod project;
mod project_cache;
mod project_manager;
pub mod project_view;

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
    confirmation_cache::ConfirmationCache,
    event_manager::EventManager,
    job_manager::{
        JobManager,
    },
    event_loop::init_event_loop,
    logic_core::{
        LogicCore, JobContext
    },
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
    loop_signal: LoopSignal,
    project_manager: ProjectManager, 
    confirmation_cache: ConfirmationCache,
    event_manager: EventManager<S>,
    job_manager: JobManager
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
            let confirmation_cache = ConfirmationCache::new();
            let event_manager = EventManager::new(event_sender);
            let job_manager = JobManager::new();

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core, 
                db_module_handler: db_module_handler,
                loop_signal: loop_signal,
                project_manager: project_manager,
                confirmation_cache: confirmation_cache,
                event_manager: event_manager,
                job_manager: job_manager
            }; 

            let _ = event_loop.run(None, &mut event_loop_resource, |resource|{
                while let Some(job) = resource.job_manager.pop_front() {
                    resource.logic_core.on_job(
                        job, 
                        JobContext { 
                            db_module_handler: &mut resource.db_module_handler, 
                            project_manager: &resource.project_manager, 
                            event_manager: &resource.event_manager, 
                            job_manager: &mut resource.job_manager, 
                            confirmation_cache: &mut resource.confirmation_cache, 
                        }
                    );
                };

                if resource.logic_core.is_shutdown() {
                    println!("logic module shutdown");
                    resource.loop_signal.stop();
                };
            }); 
        });
        
        LogicModuleHandler {
            thread_handle: Some(handle),
            logic_commands: sender,
        }
    }
}
