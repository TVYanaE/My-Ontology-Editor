use std::path::Path;

use thiserror::Error;

use super::OpenProjectEventError;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::open_project_event::OpenProjectEvent;

use crate::modules::app::app_kernel::AppEventError;

use crate::modules::app::project::Project;
use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_error::ProjectError;

pub struct LoadProjectToRAMContext {
    project_id: ProjectID,
    project: Project,     
}

#[derive(Debug, Error)]
pub enum LoadProjectToRAMError {
    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),
}


pub async fn load_project_to_ram(
    project_id: ProjectID,
    project_dir_cache: impl AsRef<Path>,
) -> Result<LoadProjectToRAMContext, LoadProjectToRAMError> {
    let project = Project::new(project_dir_cache).await?;

    Ok(
        LoadProjectToRAMContext {  
            project_id,
            project,
        }
    )
}

pub fn load_project_to_ram_callback(
    result: Result<LoadProjectToRAMContext, LoadProjectToRAMError>
) -> Option<AppEvent> {
    match result {
        Ok(context) => {
            Some(
                AppEvent::OpenProjectEvent(
                    OpenProjectEvent::ProjectLoadedToRAM { 
                        project_id: context.project_id,
                        project: context.project, 
                    }
                )
            ) 
        },
        Err(error) => {
            match error {
                LoadProjectToRAMError::ProjectError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::OpenProjectEventError(
                                OpenProjectEventError::ProjectError(error)
                            )
                        )
                    )
                } 
            } 
        },
    }
}
