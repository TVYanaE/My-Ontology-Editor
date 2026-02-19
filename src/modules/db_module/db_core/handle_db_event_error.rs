use crate::{
    aliases::{
        DBEvents
    },
};
use super::{
    db_core_logic::DBEventError,
};

pub fn handle_db_event_error(
    db_events: &DBEvents,
    error: DBEventError,
) {
    match error {
        // TODO Logic for error handling  
    }
}
