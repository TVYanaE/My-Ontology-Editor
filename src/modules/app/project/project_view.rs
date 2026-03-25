
use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::semantic_node::SemanticNode;

pub struct ProjectView {
    project_id: ProjectID, 
    project_name: String,
    semantic_nodes: Vec<SemanticNode>
}

impl ProjectView {
    pub fn new(
        project_id: ProjectID,
        project_name: String,
    ) -> Self {
        let semantic_nodes = Vec::with_capacity(64);

        Self { 
            project_id, 
            project_name, 
            semantic_nodes, 
        }
    } 
    pub fn get_project_name(&self) -> &str {
        &self.project_name
    }
    pub fn get_project_id(&self) -> ProjectID {
        self.project_id.clone()
    }
    pub fn iter_semantic_nodes(&self) -> impl Iterator<Item = &SemanticNode> {
        self.semantic_nodes.iter()
    }
}
