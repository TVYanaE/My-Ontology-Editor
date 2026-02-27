
use crate::{
    modules::{
        db_module::{
            DBCommand, DBConnectHandlerID,
        },
    },
};
use super::{
    super::{
        project_error::ProjectError,
    },
    ProjectLogic,
};


impl ProjectLogic {
    pub fn sync_project_data_from_db(
        db_connect_handler_id: DBConnectHandlerID,  
    ) -> Result<(), ProjectError> {
         


        Ok(())
    }
}
