use std::{
    path::Path
};
use rusqlite::{
    Connection,
};
use super::{
    super::{
        db_connect_cache::DBConnectCache,
        db_connect_handler::{
            DBConnectHandlerID, DBConnectHandler,
        },
        db_core_error::DBCoreError,
        Migrations,
    }, 
    DBCoreLogic
};


impl DBCoreLogic {
    pub fn create_db_file(
        db_file_path: &impl AsRef<Path>,
        migrations: Option<Migrations>,
        db_connect_cache: &mut DBConnectCache,
    ) -> Result<DBConnectHandlerID, DBCoreError> {
        let db_connection = Connection::open(db_file_path)?;

        // Setting for foreign keys 
        db_connection.pragma_update(None, "foreign_keys", 1)?;

        // Do migrations 
        if let Some(migrations) = migrations {
            for migration in migrations {
                db_connection.execute(&migration, ())?;
            }
        }; 

        let db_connect_handler_id = DBConnectHandlerID::new();
        let db_connect_handler = DBConnectHandler {
            connection: db_connection,
        };

        db_connect_cache.push(db_connect_handler_id.clone(), db_connect_handler);

        Ok(db_connect_handler_id) 
    }      
}
