use std::{
    collections::{
        HashMap,
    },
};

use super::{
    project::{
        ProjectID, Project,
    },
};

pub struct ProjectCache {
    projects: HashMap<ProjectID, Project>
}

impl ProjectCache {
    pub fn new() -> Self {
        Self { 
            projects: HashMap::with_capacity(2), 
        }
    }

    pub fn push(
        &mut self,
        project_id: ProjectID,
        project: Project,
    ) {
        self.projects.insert(project_id, project);
    }

    pub fn get_project_ref(
        &self, 
        project_id: &ProjectID
    ) -> Option<&Project> {
        self.projects.get(&project_id)
    }

    pub fn get_project_mut_ref(
        &mut self,
        project_id: &ProjectID
    ) -> Option<&mut Project> {
        self.projects.get_mut(project_id)
    }
}
