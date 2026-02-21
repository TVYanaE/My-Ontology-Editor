use super::{
    super::events::DBEvents,
    DBEvent,
    db_core_logic::DBEventError,
};


pub fn handle_db_event_error(
    db_events: &DBEvents,
    error: DBEventError,
) {
    match error {
        DBEventError::ProjectDBError(_) => {
            // TODO: Logic for sending error to Logic Module
            db_events.send(DBEvent::Shutdown).expect("Event Loop critical error. DB Module");
        },
    }
}
