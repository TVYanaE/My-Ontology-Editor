mod confirmation_cache;
mod db_core;
mod event_loop;
mod job_manager;
mod logic_core;
mod logic_module_handler;
mod logic_module_io;
pub mod prelude;
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
    }, 
};
use self::{
    db_core::DBCore,
    confirmation_cache::ConfirmationCache,
    logic_module_io::{
        event_manager::{
            EventManager,
        },
        logic_command::{
            LogicCommand,
        },
        event_sender::{
            EventSender,
        },
    },
    job_manager::{
        JobManager,
    },
    event_loop::init_event_loop,
    logic_core::{
        LogicCore, JobContext
    }, 
    logic_module_handler::LogicModuleHandler,
    project_cache::ProjectCache,
    project_manager::ProjectManager,
};

struct EventLoopResource<S> 
where 
    S: EventSender + Send + 'static
{
    logic_core: LogicCore,
    db_core: DBCore, 
    loop_signal: LoopSignal,
    project_manager: ProjectManager, 
    confirmation_cache: ConfirmationCache,
    event_manager: EventManager<S>,
    job_manager: JobManager,
    project_cache: ProjectCache,
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
            let project_manager = ProjectManager::new(app_dirs);
            let confirmation_cache = ConfirmationCache::new();
            let event_manager = EventManager::new(event_sender);
            let job_manager = JobManager::new();
            let project_cache = ProjectCache::new();
            let db_core = DBCore::new();

            let mut event_loop_resource = EventLoopResource {
                logic_core: logic_core, 
                db_core: db_core,
                loop_signal: loop_signal,
                project_manager: project_manager,
                confirmation_cache: confirmation_cache,
                event_manager: event_manager,
                job_manager: job_manager,
                project_cache: project_cache,
            }; 

            let _ = event_loop.run(None, &mut event_loop_resource, |resource|{
                while let Some(job) = resource.job_manager.pop_front() {
                    resource.logic_core.on_job(
                        job, 
                        JobContext { 
                            db_core: &mut resource.db_core, 
                            project_manager: &resource.project_manager, 
                            event_manager: &resource.event_manager, 
                            job_manager: &mut resource.job_manager, 
                            confirmation_cache: &mut resource.confirmation_cache, 
                            project_cache: &mut resource.project_cache,
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
