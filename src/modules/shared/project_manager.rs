mod project;
mod project_layouts;
mod project_manager_logic;
mod project_main_files_payloads;

use std::{
    path::PathBuf
};
use thiserror::{
    Error,
};
use self::{
    project_manager_logic::{
        CreateProjectContext,
        ProjectManagerLogic,
    },
    project::Project,
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
    state: ProjectManagerState
}

impl ProjectManager {
    pub fn new() -> Self {
        Self { 
            state: ProjectManagerState::default(),
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
                project_dir: descriptor.project_dir 
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
}
