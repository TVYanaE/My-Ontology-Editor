mod handle_logic_event_error;
mod logic_core_logic;

use std::{
    sync::{
        Arc, RwLock,
    },
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic_module::{
            events::{
                LogicEvent
            },
        },
        shared::{
            project_manager::ProjectManager,
        },
    },
};
use super::{
    CustomEvents, LogicEvents,
    ExternalEvent,
};
use self::{
    handle_logic_event_error::handle_logic_event_error,
    logic_core_logic::LogicCoreLogic,
};

#[derive(Debug)]
pub enum LogicCoreState {
    Wait, 
    Shutdown,
    Processing,
}

impl Default for LogicCoreState {
    fn default() -> Self {
        Self::Wait
    }
}

pub struct LogicCore {
    logic_core_state: LogicCoreState,  
}

impl LogicCore {
    pub fn new() -> Self {
        Self {
            logic_core_state: LogicCoreState::default(),
        }
    }
    
    pub fn on_event(
        &mut self, 
        event: LogicEvent,
        custom_events: &CustomEvents,
        logic_events: &LogicEvents,
        app_dirs: &ApplicationDirectories, 
        project_manager: Arc<RwLock<ProjectManager>>
    ) {
        let current_state = std::mem::replace(
            &mut self.logic_core_state, 
            LogicCoreState::Processing
        ); 

        self.logic_core_state = match (current_state, event) {
            (LogicCoreState::Wait, event) => {
                match LogicCoreLogic::logic_event_handle(
                    event, 
                    app_dirs, 
                    custom_events,
                    logic_events, 
                    project_manager,
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Wait,
                    Err(error) => {
                        handle_logic_event_error(error, logic_events, custom_events); 
                        LogicCoreState::Wait
                    },
                }              
            },
            (current_state,_) => current_state,
        }
    }

    pub fn event_loop_closed_handle(&mut self) {
        self.logic_core_state = LogicCoreState::Shutdown;
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.logic_core_state {
            LogicCoreState::Shutdown => true,
            _ => false
        }
    }
}
