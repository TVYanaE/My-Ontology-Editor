use std::collections::HashMap;

use crate::modules::app::project::project_view::ProjectView;
use crate::modules::app::project::project_id::ProjectID;

pub struct ProjectViewManager {
    project_views: HashMap<ProjectID, ProjectView>,
}

impl ProjectViewManager {
    pub fn new() -> Self {
        Self { 
            project_views: HashMap::with_capacity(4), 
        }
    }
    pub fn push(
        &mut self,
        id: ProjectID,
        view: ProjectView,
    ) {
        if !self.project_views.contains_key(&id) {
            self.project_views.insert(id, view);
        }
    }
    pub fn get(
        &self,
        id: &ProjectID
    ) -> Option<&ProjectView> {
        self.project_views.get(id)
    }
    pub fn get_mut(
        &mut self,
        id: &ProjectID
    ) -> Option<&mut ProjectView> {
        self.project_views.get_mut(id)
    }
    pub fn get_iter(&self) -> impl Iterator<Item = (&ProjectID, &ProjectView)> {
        self.project_views.iter()
    }
    pub fn get_iter_mut(&mut self) -> impl Iterator<Item = (&ProjectID, &mut ProjectView)> {
        self.project_views.iter_mut()
    }
}
