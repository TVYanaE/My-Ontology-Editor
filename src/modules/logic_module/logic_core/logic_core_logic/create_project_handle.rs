use std::{
    path::PathBuf,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
    },
};
use super::{
    CustomEvents,
    super::{
        super::{
            project_manager::{ 
                ProjectManager, CreateProjectDescriptor
            },
        },
    },
    LogicEventError
};

pub struct CreateProjectContext<'c> {
    pub app_dirs: &'c ApplicationDirectories,
    pub project_name: String,
    pub project_dir: PathBuf,
    pub project_manager: &'c mut ProjectManager,
    pub custom_events: &'c CustomEvents,
}

pub fn create_project_handle(
    context: CreateProjectContext,
) -> Result<(), LogicEventError> { 
    context.project_manager.create_project(
        CreateProjectDescriptor { 
            project_name: context.project_name, 
            project_dir: context.project_dir, 
            projects_dir_cache_path: context.app_dirs.cache_directory.projects_dir.clone(),
        },
        context.custom_events
    )?;


    Ok(())
}
