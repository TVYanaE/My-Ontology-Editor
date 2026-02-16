use std::{
    time::Duration,
    thread::sleep,
};
use tracing::{
    instrument
};
use thiserror::{
    Error
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        graphics_module::{
            CustomEvent, ExternalEvent,
        },
    },
};
use super::{
    super::{
        LogicEvents, CustomEvents,
        events::{
            LogicEvent, ProjectDescriptor,
        }
    },
    LogicCoreState,
};

pub struct LogicCoreLogic;

#[derive(Debug, Error)]
pub enum LogicEventError {
    #[error("Winit Event Loop is closed: {0}")]
    EventLoopClosed(#[from] winit::event_loop::EventLoopClosed<CustomEvent>),

    #[error("MPSC Channel was closed {0}")]
    MPSCChannelError(#[from] std::sync::mpsc::SendError<LogicEvent>),
}

impl LogicCoreLogic {
    #[instrument(skip_all,err)]
    pub fn logic_event_handle(
        event: LogicEvent,
        app_dirs: &ApplicationDirectories,
        custom_events: &CustomEvents,
        logic_events: &LogicEvents,
    ) -> Result<Option<LogicCoreState>, LogicEventError> {
        match event {
            LogicEvent::CreateProject(project_descriptor) => {
                handle_create_project_event(
                    project_descriptor, 
                    app_dirs
                )?;
                println!("got task"); 
                sleep(Duration::from_secs(1));
                println!("done task");
                  
                logic_events.send(LogicEvent::ProjectCreated)?; 
                Ok(None)
            },
            LogicEvent::ProjectCreated => { 
                custom_events.send_event(ExternalEvent::TaskDone.into())?;
                Ok(None)
            }
            LogicEvent::Shutdown => {
                // Logic for Graceful Shutdown  
                Ok(Some(LogicCoreState::Shutdown))
            }
        }
    } 
}

fn handle_create_project_event(
    _project_descriptor: ProjectDescriptor,
    _app_dirs: &ApplicationDirectories
) -> Result<(), LogicEventError> {
     
    Ok(())
}
