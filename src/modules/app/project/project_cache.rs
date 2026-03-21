use std::path::{Path, PathBuf};
use std::collections::hash_map::HashMap;

use crate::modules::app::project::project_id::ProjectID;

pub struct ProjectCache {
    projects: HashMap<ProjectID, PathBuf>, 
}

impl ProjectCache {
    pub fn new() -> Self {
        Self { 
            projects: HashMap::with_capacity(64),
        }
    } 

    pub fn add_project<P>(
        &mut self,
        project_id: &ProjectID,
        project_dir_cache: &P
    ) 
    where 
        P: AsRef<Path>
    { 
        self.projects.insert(
            project_id.clone(), 
            project_dir_cache.as_ref().to_path_buf()
        );
    }

    pub fn get_by_id(
        &self,
        project_id: &ProjectID,
    ) -> Option<PathBuf> {
        self.projects.get(project_id).cloned()
    }
}
