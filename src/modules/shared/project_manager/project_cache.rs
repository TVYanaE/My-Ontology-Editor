use std::{
    collections::{
        hash_map::{
            HashMap,
        },
    },
    path::{PathBuf, Path},
};
use uuid::{
    Uuid,
};

#[derive(Hash, PartialEq, Eq)]
pub struct ProjectID(Uuid);

pub struct ProjectDescriptor{
    pub name: String,
    pub full_path: PathBuf 
}

pub struct ProjectCache {
    projects_table: HashMap<ProjectID, ProjectDescriptor>
}

impl ProjectCache {
    pub fn new(
    ) -> Self {
        let projects_table = HashMap::with_capacity(8); 
        // TODO: Logic for searching of projects in cache. 

        Self { 
            projects_table: projects_table, 
        }
    }

    pub fn push(
        &mut self,
        project_id: ProjectID,
        project_descriptor: ProjectDescriptor,
    ) {
        // TODO: Push Logic
        let _ = self.projects_table.insert(project_id, project_descriptor);
    }
}


