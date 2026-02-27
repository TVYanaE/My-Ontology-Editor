use uuid::{
    Uuid,
};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct SemanticNodeID(Uuid);

impl SemanticNodeID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn create_from(id: Uuid) -> Self {
        Self(id)
    }
}

pub struct SemanticNode {
    pub id: SemanticNodeID,
    pub name: String,
}
