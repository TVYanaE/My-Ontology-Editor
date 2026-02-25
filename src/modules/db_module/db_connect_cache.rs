use std::{
    collections::{
        HashMap,
    },
};
use super::{
    db_connect_handler::{
        DBConnectHandler, DBConnectHandlerID
    }, 
};

pub struct DBConnectCache {
    db_connections: HashMap<DBConnectHandlerID, DBConnectHandler>,
}

impl DBConnectCache {
    pub fn new() -> Self {
        Self { 
            db_connections: HashMap::with_capacity(4), 
        }
    }

    pub fn push(
        &mut self,
        db_connect_handler_id: DBConnectHandlerID,
        db_connect_handler: DBConnectHandler,
    ) {
        self.db_connections.insert(
            db_connect_handler_id, 
            db_connect_handler
        );
    }

    pub fn remove(
        &mut self,
        db_connect_handler_id: DBConnectHandlerID
    ) -> Option<DBConnectHandler>{

        self.db_connections.remove(&db_connect_handler_id)
    }
}
