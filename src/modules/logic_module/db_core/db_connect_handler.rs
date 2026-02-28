use uuid::{
    Uuid,
};
use rusqlite::{
    Connection,
};

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct DBConnectHandlerID(Uuid);

impl DBConnectHandlerID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct DBConnectHandler {
    pub connection: Connection,
}
