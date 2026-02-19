mod db_core_logic;
mod handle_db_event_error;

use super::{
    DBEvent, DBEvents,
};
use self::{
    db_core_logic::DBCoreLogic,
    handle_db_event_error::handle_db_event_error,
};

enum DBCoreState {
    Wait, 
    Shutdown,
    Processing,
}

impl Default for DBCoreState {
    fn default() -> Self {
        Self::Wait
    }
}

pub struct DBCore {
    state: DBCoreState
}

impl DBCore {
    pub fn new() -> Self {
        Self { 
            state: DBCoreState::default() 
        }
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.state {
            DBCoreState::Shutdown => true,
            _ => false
        }
    }

    pub fn on_event(
        &mut self,
        event: DBEvent,
        db_events: &DBEvents,
    ) {
        let current_state = std::mem::replace(
            &mut self.state, 
            DBCoreState::Processing
        );

        self.state = match (current_state, event) {
            (DBCoreState::Wait, event) => {
                match DBCoreLogic::db_event_handle(event) {
                    Ok(Some(new_state)) => new_state, 
                    Ok(None) => DBCoreState::Wait,
                    Err(error) => {
                        handle_db_event_error(db_events, error); 
                        DBCoreState::Wait
                    },
                }      
            },
            (current_state,_) => current_state
        };
    }
}
