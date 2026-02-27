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

    pub fn remove(
        &mut self,
        project_id: &ProjectID
    ) -> Option<Project> {
        self.projects.remove(project_id)
    }

    pub fn with_project<F, R, E>(
        &self, 
        project_id: &ProjectID,
        func: F
    ) -> Option<Result<R, E>>
    where 
        F: FnOnce(&Project) -> Result<R, E>
    {
        self.projects.get(project_id).map(func)
    }

    pub fn with_project_mut<F, R, E>(
        &mut self,
        project_id: &ProjectID,
        func: F,
    ) -> Option<Result<R, E>>
    where 
        F: FnOnce(&mut Project) -> Result<R, E>
    {
        self.projects.get_mut(project_id).map(func)
    }
}
