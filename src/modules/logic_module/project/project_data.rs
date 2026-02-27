pub mod semantic_node; 
pub mod semantic_nodes_relation;

use std::{
    collections::HashMap,
};
use self::{
    semantic_node::{
        SemanticNode, SemanticNodeID,
    },
    semantic_nodes_relation::{
        SemanticNodesRelationID, RelatedSemanticNodes, 
    },
};

pub struct ProjectData {
    pub semantic_nodes: HashMap<SemanticNodeID, SemanticNode>, 
    pub semantic_nodes_relations: HashMap<SemanticNodesRelationID, RelatedSemanticNodes>
}

impl Default for ProjectData {
    fn default() -> Self {
        Self { 
            semantic_nodes: HashMap::with_capacity(32), 
            semantic_nodes_relations: HashMap::with_capacity(32), 
        }
    }
}
