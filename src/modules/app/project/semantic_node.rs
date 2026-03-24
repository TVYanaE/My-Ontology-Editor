use std::str::FromStr;

use uuid::Uuid;
use uuid::Error;

#[derive(Debug, Clone)]
pub struct SemanticNode {
    id: SemanticNodeID, 
    name: String,
}

impl SemanticNode {
    pub fn new(name: &str) -> Self {
        let id = SemanticNodeID::new();

        Self { 
            id, 
            name: name.to_string() 
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_id(&self) -> SemanticNodeID {
        self.id.clone()
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct SemanticNodeID(Uuid);

impl SemanticNodeID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn as_str(&self) -> String {
        self.0.to_string()
    }
    pub fn from_str(str: &str) -> Result<Self, Error> {
        let uuid = Uuid::from_str(str)?;
        Ok(Self(uuid))
    }
}
