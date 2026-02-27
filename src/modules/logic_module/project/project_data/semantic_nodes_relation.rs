use uuid::{
    Uuid,
};
use super::{
    semantic_node::SemanticNodeID,
};

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct SemanticNodesRelationID(Uuid);

impl SemanticNodesRelationID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    pub fn create_from(id: Uuid) -> Self {
        Self(id)
    }
}

pub struct RelatedSemanticNodes(SemanticNodeID, SemanticNodeID);

impl RelatedSemanticNodes {
    pub fn create_from(f_id: Uuid, s_id: Uuid) -> Self {
        Self (
            SemanticNodeID::create_from(f_id), 
            SemanticNodeID::create_from(s_id),
        )
    }
}
