use uuid::{
    Uuid,
};


pub struct SemanticNodeID(Uuid);

impl SemanticNodeID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub struct SemanticNode {
    pub id: SemanticNodeID,
    pub name: String,
    pub connected_nodes: Vec<SemanticNodeID>,
}
