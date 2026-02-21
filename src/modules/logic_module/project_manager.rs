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
    modules::{
        graphics_module::{CustomEvent, CustomEvents},
        db_module::DBEvents,
    },
};
use self::{
    project::{    
        Project, ProjectError,
    },
    project_cache::ProjectCache,
    project_manager_logic::{
        CreateUnpackedProjectContext,
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
        custom_events: &CustomEvents,
    ) -> Result<(), ProjectManagerError> {
        // TODO Logic for check alredy opened project 
         
        let project = ProjectManagerLogic::create_unpacked_project(
            CreateUnpackedProjectContext { 
                projects_dir_cache_path: descriptor.projects_dir_cache_path, 
                project_name: descriptor.project_name, 
                project_dir: descriptor.project_dir,
                db_events: self.db_events.clone(),
                custom_events: custom_events,
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

    #[error("Winit Event Loop closed: {0}")]
    WinitEventLoopError(#[from] winit::event_loop::EventLoopClosed<CustomEvent>),

    #[error("One Shot Recv Error")]
    OneShotRecvErro(#[from] oneshot::RecvError)
}
