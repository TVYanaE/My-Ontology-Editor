pub mod db_core_error;
mod db_core_logic;
mod db_core_state;
mod db_core_state_handle;

use crate::{
    modules::{
        db_module::{
            events::{
                DBCommand,
            },
        },
    },
};
use self::{
    db_core_state::DBCoreState,
    db_core_state_handle::DBCoreStateHandle,
};


pub struct DBCore {
    state: DBCoreState,
}

impl DBCore {
    pub fn new() -> Self {
        Self { 
            state: DBCoreState::default(),
        }
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.state {
            DBCoreState::Shutdown => true,
            _ => false
        }
    }

    pub fn on_command(
        &mut self,
        command: DBCommand,
    ) {
        let current_state = std::mem::replace(
            &mut self.state, 
            DBCoreState::Processing
        );

        self.state = match (current_state, command) {
            (DBCoreState::Ready, command) => {
                match DBCoreStateHandle::ready_handle(command) {
                    Some(new_state) => new_state,
                    None => DBCoreState::Ready,
                } 
            },
            (current_state,_) => current_state
        };
    }
}
