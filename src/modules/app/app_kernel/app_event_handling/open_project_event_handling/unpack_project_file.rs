use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{Seek, SeekFrom};

use std::sync::Arc;

use thiserror::Error;

use tar::Archive;

use crate::modules::consts::PROJECT_FILE_HEADER_SIZE;

use super::OpenProjectEventError;

use super::super::super::super::app_event::AppEvent;
use super::super::super::super::app_event::open_project_event::OpenProjectEvent;

use super::super::super::super::project::project_id::ProjectID;

use super::super::super::super::app_dirs::AppDirs;

use super::super::app_event_handling_error::AppEventHandlingError;

#[derive(Debug, Error)]
pub enum UnpackProjectFileError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),    
}

#[derive(Debug)]
pub struct UnpackProjectFileContext {
    project_id: ProjectID,
    project_dir_cache: PathBuf,

}

pub fn unpack_project_file(
    project_file_path: impl AsRef<Path>,
    project_id: ProjectID,
    app_dirs: Arc<AppDirs>,
) -> Result<UnpackProjectFileContext, UnpackProjectFileError> {
    // Create directory for the project in cache dir 
    let project_dir_cache = app_dirs
        .cache_directory
        .projects_dir_path
        .join(project_id.get_str());

    println!("Project Dir in cache: {:?}", project_dir_cache);
    std::fs::create_dir(&project_dir_cache)?; 

    // Unpack Project File  
    let mut project_file = File::open(project_file_path)?;  

    // Move cursor 
    project_file.seek(SeekFrom::Start(PROJECT_FILE_HEADER_SIZE as u64))?;

    let mut archive = Archive::new(project_file); 
    archive.unpack(&project_dir_cache)?;

    Ok(
        UnpackProjectFileContext {  
            project_id: project_id,
            project_dir_cache: project_dir_cache,
        }
    )
}

pub fn unpack_project_file_callback(
    result: Result<UnpackProjectFileContext, UnpackProjectFileError>
) -> Option<AppEvent> {
    match result {
        Ok(context) => {
            Some(
                AppEvent::OpenProjectEvent(
                    OpenProjectEvent::PushProjectToCache { 
                        project_id: context.project_id, 
                        project_dir_cache: context.project_dir_cache,
                    }
                )
            ) 
        },
        Err(error) => {
            match error {
                UnpackProjectFileError::STDIOError(error) => {
                    Some(
                        AppEvent::KernelError(
                            AppEventHandlingError::OpenProjectEventError(
                                OpenProjectEventError::STDIOError(error)    
                            ).into()
                        )
                    )
                },
            } 
        },
    }
}
