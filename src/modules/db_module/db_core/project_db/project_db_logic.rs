use std::{
    path::{
        Path,
    },
};
use rusqlite::{
    Connection,
};
use super::{
    ProjectDBError
};

pub struct ProjectDBLogic;

impl ProjectDBLogic {
    pub fn open_connection_handle(
        project_root_path: &impl AsRef<Path>
    ) -> Result<Connection, ProjectDBError> {
        let mut db_path = project_root_path.as_ref().to_path_buf();
        db_path.push("topology");
        db_path.set_extension("db3");
       
        let connection = Connection::open(db_path)?; 

        connection.execute(
            "CREATE TABLE person (
                id   INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB
            )",
            (), // empty list of parameters.
        )?;

        // TODO: Logic for migration

        Ok(connection)
    } 
}
