mod create_project_handle;

use tracing::{
    instrument, error,
};
use thiserror::{
    Error
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        db_module::DBEvent,
        graphics_module::{
            CustomEvent, ExternalEvent,
        },
        shared::{
            db_module_handler::DBModuleHandler, 
        },
    },
};
use super::{
    super::{
        LogicEvents, CustomEvents,
        events::{
            LogicEvent,
        },
        project_manager::{
            ProjectManager, 
            ProjectManagerError,
        },
    },
    LogicCoreState,
};
use self::{
    create_project_handle::{
        create_project_handle,
        CreateProjectContext,
    },
};

pub struct LogicCoreLogic;

#[derive(Debug, Error)]
pub enum LogicEventError {
    #[error("Winit Event Loop is closed: {0}")]
    EventLoopClosed(#[from] winit::event_loop::EventLoopClosed<CustomEvent>),

    #[error("MPSC Channel was closed {0}")]
    MPSCChannelLogicEventError(#[from] std::sync::mpsc::SendError<LogicEvent>),

    #[error("MPSC Channel was closed {0}")]
    MPSCChannelDBEventError(#[from] std::sync::mpsc::SendError<DBEvent>),

    #[error("Std IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Project Manager Error: {0}")]
    ProjectManagerError(#[from] ProjectManagerError),

    #[error("Rwlock poison error")]
    RwLockPoisonError
}

impl LogicCoreLogic {
    #[instrument(skip_all,err)]
    pub fn logic_event_handle(
        event: LogicEvent,
        app_dirs: &ApplicationDirectories,
        custom_events: &CustomEvents,
        logic_events: &LogicEvents,
        project_manager: &mut ProjectManager,
        db_module_handler: &mut DBModuleHandler
    ) -> Result<Option<LogicCoreState>, LogicEventError> {
        match event {
            LogicEvent::CreateProject{
                task_id,
                project_name, 
                project_dir
            } => {
                /* create_project_handle(
                    CreateProjectContext { 
                        app_dirs: app_dirs, 
                        project_name: project_name, 
                        project_dir: project_dir, 
                        project_manager: project_manager,
                        custom_events: custom_events
                    } 
                )?; */ 
                std::thread::sleep(std::time::Duration::from_secs(1));
                
                custom_events.send_event(ExternalEvent::TaskDone(task_id).into())?;
                Ok(None)
            },
            LogicEvent::Confirmation { task_id, confirm } => {
                
                Ok(None)
            }, 
            LogicEvent::Shutdown => {
                db_module_handler.db_events.send(DBEvent::Shutdown)?;
                
                if let Some(handle) = db_module_handler.thread_handle.take() {
                    // Error will come due to panic in thread 
                    if let Err(error) = handle.join() {
                        error!(error = ?error, "DB Thread Panic");                
                    }
                }

                // Logic for Graceful Shutdown  
                Ok(Some(LogicCoreState::Shutdown))
            }
        }
    } 
}


