use std::str::FromStr;

use uuid::Uuid;
use uuid::Error;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct ProjectID(Uuid); 

impl ProjectID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    } 
    pub fn from_str(id_str: &str) -> Result<Self, Error> {
        Ok(Self(Uuid::from_str(id_str)?))
    }
    pub fn get_str(&self) -> String {
        self.0.to_string()
    }
    pub fn as_bytes(&self) -> &[u8; 16] {
        self.0.as_bytes()
    }    
    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        let uuid = uuid::Uuid::from_bytes(*bytes);

        Self(uuid)
    }
}
