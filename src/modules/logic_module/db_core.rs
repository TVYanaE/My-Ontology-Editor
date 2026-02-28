mod db_connect_cache;  
mod db_connect_handler;
pub mod db_core_error;
mod db_core_logic;

use std::{
    path::{
        Path,
    },
};
use crate::{ 
    migrations::{
        SEMANTIC_NODES_TABLE_MIGRATION,
        SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION
    }, 
};
use self::{
    db_core_logic::DBCoreLogic,
    db_connect_cache::DBConnectCache,
    db_core_error::DBCoreError,
};
pub use self::{
    db_connect_handler::DBConnectHandlerID,
};

pub type Migrations = Vec<String>;

pub struct DBCore {
    db_connect_cache: DBConnectCache,
}

impl DBCore {
    pub fn new() -> Self {
        Self {
            db_connect_cache: DBConnectCache::new(),
        }
    }

    pub fn create_db_file(
        &mut self, 
        db_file_path: &impl AsRef<Path>
    ) -> Result<DBConnectHandlerID, DBCoreError> {
        let mut migrations = Migrations::with_capacity(4);
        migrations.push(SEMANTIC_NODES_TABLE_MIGRATION.to_string());
        migrations.push(SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION.to_string());

        let connect_id = DBCoreLogic::create_db_file(
            db_file_path, 
            Some(migrations), 
            &mut self.db_connect_cache
        )?;

        Ok(connect_id)
    }
    
}
