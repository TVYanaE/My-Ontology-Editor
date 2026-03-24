
use petgraph::Graph;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::semantic_node::SemanticNode;

pub struct ProjectView {
    project_id: ProjectID, 
    project_name: String,
    semantic_nodes: Graph<SemanticNode, String, petgraph::Directed>
}

impl ProjectView {
    pub fn new(
        project_id: ProjectID,
        project_name: String,
    ) -> Self {
        let root_node = SemanticNode::new("I");

        let mut semantic_nodes = Graph::with_capacity(32, 64); 

        semantic_nodes.add_node(root_node);

        Self { 
            project_id, 
            project_name, 
            semantic_nodes, 
        }
    } 
    pub fn get_project_name(&self) -> &str {
        &self.project_name
    }
}
