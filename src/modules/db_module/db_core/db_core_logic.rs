use std::{
    path::Path
};
use rusqlite::{
    Connection as DBConnection,
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
    db_core_state::DBCoreState,
    db_core_error::DBCoreError,
};

pub struct DBCoreLogic;

impl DBCoreLogic {
    pub fn create_db_file(
        db_file_path: &impl AsRef<Path>,
        migration: Option<String>,
        response_target: OneShotSender<Result<(), DBCoreError>>
    ) -> Option<DBCoreState> {
        let db_connection = match DBConnection::open(db_file_path) {
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

        if let Some(migration) = migration {
            match db_connection.execute(
                &migration, 
                ()
            ) {
                Ok(_) => return None,
                Err(error) => {
                    match response_target.send(Err(DBCoreError::RuSQlitError(error))) {
                        Ok(_) => return None,
                        Err(_) => return None,
                    }
                },
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
