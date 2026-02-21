
use thiserror::{
    Error,
};
use tracing::{
    instrument,
};
use super::{
    project_db::{
        ProjectDBError,
        ProjectDB,
    },
    DBEvent,
    DBCoreState,
}; 


pub struct DBCoreLogic;

#[derive(Debug, Error)]
pub enum DBEventError {
    #[error("ProjectDBError: {0}")]
    ProjectDBError(#[from] ProjectDBError)
    
}

impl DBCoreLogic {
    #[instrument(skip_all,err)]
    pub fn db_event_handle(
        event: DBEvent,
        project_db: &mut ProjectDB,
    ) -> Result<Option<DBCoreState>, DBEventError> {
        match event {
            DBEvent::Shutdown => {
                println!("DB Thread shutdown");
                Ok(Some(DBCoreState::Shutdown))
            },
            DBEvent::OpenConnection{
                project_root_path, 
            } => {
                project_db.open_connection(
                    &project_root_path,
                )?;
                Ok(None)
            },
        } 
    } 
}
