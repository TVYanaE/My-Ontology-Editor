use std::{
    path::Path
};
use rusqlite::{
    Connection,
};
use tracing::{
    error,
};
use crate::{
    aliases::{
        OneShotSender
    },
};
use super::{
    super::{
        super::{
            commands::Migrations,
        },
        db_core_state::DBCoreState,
        db_core_error::DBCoreError,
    }, 
    DBCoreLogic
};


impl DBCoreLogic {
    pub fn create_db_file(
        db_file_path: &impl AsRef<Path>,
        migration: Option<Migrations>,
        response_target: OneShotSender<Result<(), DBCoreError>>
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

        if let Some(migrations) = migration {
            for migration in migrations {
                match db_connection.execute(&migration, ()) {
                    Ok(_) => {},
                    Err(error) => {
                        match response_target.send(Err(DBCoreError::RuSQlitError(error))) {
                            Ok(_) => return None,
                            Err(_) => return None,
                        }
                    }
                }
            }
        }; 

        match response_target.send(Ok(())) {
            Ok(_) => return None,
            Err(error) => {
                error!(error=?error, "Send Error DB module");
                return None;
            },
        }
    }     
 
}
