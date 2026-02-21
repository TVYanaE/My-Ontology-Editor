use std::{
    path::{
        Path,
    },
};
use rusqlite::{
    Connection,
};
use oneshot::{
    Sender,
};
use super::{
    ProjectDBError
};

pub struct ProjectDBLogic;

impl ProjectDBLogic {
    pub fn open_connection_handle(
        project_root_path: &impl AsRef<Path>,
    ) -> Result<Connection, ProjectDBError> {
        let mut db_path = project_root_path.as_ref().to_path_buf();
        db_path.push("project_db");
        db_path.set_extension("db3");
       
        let connection = Connection::open(db_path)?; 

        // Migrations
        connection.execute(
            r#"
                CREATE TABLE IF NOT EXISTS semantic_nodes (
                    id TEXT NOT NULL PRIMARY KEY,
                    name TEXT
                )
            "#, 
            ()
        )?;

        //response_target.send(Ok(()))?; 
        
        Ok(connection)
    } 
}
