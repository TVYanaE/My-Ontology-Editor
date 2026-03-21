use std::path::Path;

use thiserror::Error;

use crate::modules::app::app_event::AppEvent;

use crate::modules::app::project::Project;
use crate::modules::app::project::project_error::ProjectError;

pub struct LoadProjectToRAMContext {
    
}

#[derive(Debug, Error)]
pub enum LoadProjectToRAMError {
    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),
}


pub async fn load_project_to_ram(
    project_dir_cache: impl AsRef<Path>,

) -> Result<LoadProjectToRAMContext, LoadProjectToRAMError> {

    Ok(
        LoadProjectToRAMContext {  
        }
    )
}

pub fn load_project_to_ram_callback(
    result: Result<LoadProjectToRAMContext, LoadProjectToRAMError>
) -> Option<AppEvent> {

    None
}
