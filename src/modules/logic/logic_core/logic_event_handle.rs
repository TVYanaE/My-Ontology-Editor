
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
            events::CustomEvents,
        },
    },
};

pub struct LogicEventContext<'c> {
    pub app_dirs: &'c ApplicationDirectories,
    pub custom_events: &'c CustomEvents,
}

#[instrument(skip_all,err)]
pub fn logic_event_handle(
    event: LogicEvent,
    logice_event_context: LogicEventContext,
) -> Result<Option<LogicCoreState>, LogicEventError> {

    match event {
        LogicEvent::CreateProject(project_descriptor) => {
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
