use uuid::Uuid;
use serde::{
    Serialize, Deserialize
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub struct ProjectID(Uuid);

impl ProjectID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    } 
}

impl ToString for ProjectID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
