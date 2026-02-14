use std::{
    time::Duration,
};
use tracing::{
    instrument
};
use thiserror::{
    Error,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic::{
            events::{
                LogicEvent
            },
            logic_core::{
                LogicCoreState,
            },
        },
        graphics::{
            events::{
                graphics_event::{ExternalEvent},
                CustomEvents, 
            },
        },
    },
};

pub struct LogicEventContext<'c> {
    pub app_dirs: &'c ApplicationDirectories,
    pub custom_events: &'c CustomEvents,
}

#[instrument(skip_all,err)]
pub fn handle_logic_event(
    event: LogicEvent,
    logice_event_context: LogicEventContext,
) -> Result<Option<LogicCoreState>, LogicEventError> {

    match event {
        LogicEvent::CreateProject(project_descriptor) => {
            std::thread::sleep(Duration::from_secs(4));
            println!("From separeted thread");
            logice_event_context.custom_events.send_event(ExternalEvent::TaskDone.into()).unwrap();
            Ok(None)
        },
        LogicEvent::Shutdown => {
            // Logic for Graceful Shutdown  
            Ok(Some(LogicCoreState::Shutdown))
        }
    }
}

#[derive(Debug, Error)]
pub enum LogicEventError {
    
}
