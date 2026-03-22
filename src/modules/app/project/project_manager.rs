pub mod project_manager_error;

use std::collections::HashMap; 

use crate::modules::app::project::Project;
use crate::modules::app::project::project_id::ProjectID;

pub struct ProjectManager {
    projects: HashMap<ProjectID, Project>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self { 
            projects: HashMap::with_capacity(4), 
        }
    }
    pub fn push(&mut self, project_id: ProjectID, project: Project) {
        self.projects.insert(project_id, project);
    }
}
