
use std::{
    path::{PathBuf, Path},
}; 
use thiserror::{
    Error,
};
use crate::{
    aliases::{
        DBEvents,
    },
    modules::{
        db_module::DBEvent,
    },
};


/// Descriptor of unpacked project
pub struct Project {
    project_dirs_map: ProjectDirsMap,
    db_events: DBEvents,
}

impl Project {
    pub fn new(
        project_root_path: &impl AsRef<Path>,
        semantic_nodes_dir_path: &impl AsRef<Path>,
        project_meta_file_path: &impl AsRef<Path>,
        db_events: DBEvents,
    ) -> Result<Self, ProjectError> {
        let project_dirs_map = ProjectDirsMap {
            semantic_nodes_dir_path: semantic_nodes_dir_path.as_ref().to_path_buf(),
            project_meta_file_path: project_meta_file_path.as_ref().to_path_buf(),
        };
        
        db_events.send(DBEvent::OpenConnection(project_root_path.as_ref().to_path_buf())).unwrap();
        
        Ok(Self { 
            project_dirs_map: project_dirs_map, 
            db_events: db_events,
        })

    }
}

struct ProjectDirsMap {
    pub semantic_nodes_dir_path: PathBuf,
    pub project_meta_file_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectError {

}
