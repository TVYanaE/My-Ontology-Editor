use std::{
    path::PathBuf,
};


pub struct ProjectDirsLayout {
    pub semantic_nodes_catalog: SemanticNodesCatalog,
}

impl ProjectDirsLayout {
    pub fn create_defaul_dirs_layout(
    ) -> Self {
        Self { 
            semantic_nodes_catalog: SemanticNodesCatalog::default(), 
        }
    } 
}

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

