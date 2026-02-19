mod project;
mod project_cache;
mod project_layouts;
mod project_manager_logic;
mod project_main_files_payloads;

use std::{ 
    path::PathBuf
};
use thiserror::{
    Error,
};
use crate::{
    aliases::{
        DBEvents,
    },
};
use self::{
    project::{    
        Project, ProjectError,
    },
    project_cache::ProjectCache,
    project_manager_logic::{
        CreateProjectContext,
        ProjectManagerLogic,
    },
};

enum ProjectManagerState {
    ProjectNotOpened,
    ProjectOpened(Project)
}

impl Default for ProjectManagerState {
    fn default() -> Self {
        Self::ProjectNotOpened
    }
}


pub struct ProjectManager {
    state: ProjectManagerState,
    db_events: DBEvents,
    project_cache: ProjectCache, 
}

impl ProjectManager {
    pub fn new(
        db_events: DBEvents,
    ) -> Self {
        Self { 
            state: ProjectManagerState::default(),
            db_events: db_events,
            project_cache: ProjectCache::new(),
        }
    }

    pub fn create_project(
        &mut self, 
        descriptor: CreateProjectDescriptor,
    ) -> Result<(), ProjectManagerError> {
        // TODO Logic for check alredy opened project 
         
        ProjectManagerLogic::create_project(
            CreateProjectContext { 
                projects_dir_cache_path: descriptor.projects_dir_cache_path, 
                project_name: descriptor.project_name, 
                project_dir: descriptor.project_dir,
                db_events: self.db_events.clone()
            } 
        )?; 

        Ok(())
    } 
} 


#[derive(Debug)]
pub struct CreateProjectDescriptor {
    pub project_name: String,
    pub project_dir: PathBuf,
    pub projects_dir_cache_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectManagerError {
    #[error("Toml crate error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("Std IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),
}
