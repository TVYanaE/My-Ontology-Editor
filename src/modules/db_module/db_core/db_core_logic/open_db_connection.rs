use std::{
    path::Path
};
use rusqlite::{
    Connection,
};
use tracing::{
    error
};
use crate::{
    aliases::OneShotSender,
};
use super::{
    super::{
        super::{
            db_connect_cache::{
                DBConnectCache,
            },
            db_connect_handler::{
                DBConnectHandlerID,
                DBConnectHandler,
            },
        },
        db_core_state::{
            DBCoreState,
        },
        db_core_error::DBCoreError,
    },
    DBCoreLogic,
};


impl DBCoreLogic { 
    pub fn open_db_connect(
        db_file_path: &impl AsRef<Path>,
        db_connect_cache: &mut DBConnectCache,
        response_target: OneShotSender<Result<DBConnectHandlerID, DBCoreError>>,
    ) -> Option<DBCoreState> {
        let db_connection = match Connection::open(db_file_path) {
            Ok(connection) => connection,
            Err(error) => {
                match response_target.send(Err(DBCoreError::RuSQlitError(error))) {
                    Ok(_) => return None,
                    Err(error) => {
                        error!(error=?error, "Send Error DB module");
                        return None;
                    },
                }
            },
        };

        // Setting for foreign keys 
        match db_connection.pragma_update(None, "foreign_keys", 1) {
            Ok(_) => {},
            Err(error) => {
                match response_target.send(Err(DBCoreError::RuSQlitError(error))) {
                    Ok(_) => return None,
                    Err(_) => return None,
                }
            } 
        }  

        let db_connect_handler_id = DBConnectHandlerID::new();
        let db_connect_handler = DBConnectHandler {
            connection: db_connection
        };

        db_connect_cache.push(db_connect_handler_id.clone(), db_connect_handler);

        match response_target.send(Ok(db_connect_handler_id)) {
            Ok(_) => return None,
            Err(error) => {
                error!(error=?error, "Send Error DB module");
                return None;
            },
        };
    } 
}
