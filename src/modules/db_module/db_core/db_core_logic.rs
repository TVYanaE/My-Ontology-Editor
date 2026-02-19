
use thiserror::{
    Error,
};
use tracing::{
    instrument,
};
use super::{
    DBEvent,
    DBCoreState,
}; 


pub struct DBCoreLogic;

#[derive(Debug, Error)]
pub enum DBEventError {
     
}

impl DBCoreLogic {
    #[instrument(skip_all,err)]
    pub fn db_event_handle(
        event: DBEvent,
    ) -> Result<Option<DBCoreState>, DBEventError> {
        match event {
            DBEvent::Shutdown => {
                println!("DB Thread shutdown");
                Ok(Some(DBCoreState::Shutdown))
            },
        } 
    } 
}
