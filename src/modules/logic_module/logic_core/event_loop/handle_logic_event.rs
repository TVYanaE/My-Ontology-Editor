mod handle_create_project_event;

use std::{
    thread,
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
        logic_module::{
            events::{
                LogicEvent
            },
            logic_core::{
                LogicCoreState,
            },
        },
        graphics_module::{ 
            CustomEvent, CustomEvents, 
            ExternalEvent, 
        },
    },
};
use self::{
    handle_create_project_event::{
        handle_create_project_event, 
        CreateProjectEventContext,
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
            handle_create_project_event(
                project_descriptor, 
                CreateProjectEventContext { 
                    app_dirs: logice_event_context.app_dirs, 
                }
            )?;
            println!("got task"); 
            thread::sleep(Duration::from_secs(2));
            println!("done task");


            logice_event_context.custom_events.send_event(ExternalEvent::TaskDone.into())?;
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
    #[error("Winit Event Loop is closed: {0}")]
    EventLoopClosed(#[from] winit::event_loop::EventLoopClosed<CustomEvent>)
}
