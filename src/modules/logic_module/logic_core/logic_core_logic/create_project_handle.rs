use std::{
    sync::{
        Arc, RwLock,
    },
    path::PathBuf,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        shared::{
            project_manager::{
                ProjectManager, CreateProjectDescriptor
            },
        },
    },
};
use super::{
    LogicEventError
};

pub struct CreateProjectContext<'c> {
    pub app_dirs: &'c ApplicationDirectories,
    pub project_name: String,
    pub project_dir: PathBuf,
    pub project_manager: Arc<RwLock<ProjectManager>>
}

pub fn create_project_handle(
    context: CreateProjectContext,
) -> Result<(), LogicEventError> {
    let mut pm_lock = context.project_manager
        .write()
        .map_err(|_|{
            LogicEventError::RwLockPoisonError
        }
    )?;  
    
    pm_lock.create_project(
        CreateProjectDescriptor { 
            project_name: context.project_name, 
            project_dir: context.project_dir, 
            projects_dir_cache_path: context.app_dirs.cache_directory.projects_dir.clone() 
        } 
    )?;


    Ok(())
}
