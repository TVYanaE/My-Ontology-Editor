use std::{
    path::PathBuf,
};

#[derive(Debug)]
pub struct ProjectDirsLayout {
    pub semantic_nodes_catalog: SemanticNodesCatalog,
}

impl Default for ProjectDirsLayout {
    fn default() -> Self {
        Self { 
            semantic_nodes_catalog: SemanticNodesCatalog::default(), 
        } 
    } 
}

#[derive(Debug)]
pub struct SemanticNodesCatalog {
    pub path: PathBuf,
}

impl Default for SemanticNodesCatalog {
    fn default() -> Self {
        let mut path = PathBuf::new();
        path.push("semantic_nodes");
        
        Self { 
            path: path, 
        }
    }
}

