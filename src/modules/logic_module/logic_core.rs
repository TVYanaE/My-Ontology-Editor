mod logic_core_logic;

use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic_module::{
            events::{
                LogicEvent
            },
        },
        graphics_module::CustomEvents,
    },
};
use self::{
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
        app_dirs: &ApplicationDirectories, 
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
                    custom_events
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Wait,
                    Err(_error) => {
                       
                        LogicCoreState::Shutdown
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





