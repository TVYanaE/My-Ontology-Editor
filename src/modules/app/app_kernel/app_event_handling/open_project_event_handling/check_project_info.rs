use std::path::PathBuf;

use thiserror::Error;

use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use crate::modules::consts::PROJECT_FILE_HEADER_SIZE;
use crate::modules::consts::MAGIC_BYTES;

use crate::modules::app::app_kernel::app_event_handling::app_event_error::AppEventError;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::open_project_event::OpenProjectEvent;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_file_header::ProjectFileHeader;

use super::OpenProjectEventError;

#[derive(Debug, Error)]
pub enum CheckProjectInfoError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("Selected Project File doesn't exist")]
    ProjectFileDoesntExists(ErrorContext),

    #[error("Selected File isn't fiel")]
    SelectedFileIsntFile(ErrorContext),
 
    #[error("Bytemuck Pod Cast Error: {0}")]
    BytemuckPodCastError(bytemuck::PodCastError),

    #[error("Selected file in wrong format")]
    WrongFormat(ErrorContext),     
}

#[derive(Debug)]
pub struct CheckProjectInfoContext {
    project_file_path: String,
    project_id: ProjectID,
}

#[derive(Debug)]
pub struct ErrorContext {
    project_file_path: String,
}

pub async fn check_project_info(
    project_file_path: String,
) -> Result<CheckProjectInfoContext, CheckProjectInfoError> {
    let mut project_file_path_buf = PathBuf::new(); 
    project_file_path_buf.push(&project_file_path);

    if !project_file_path_buf.exists() {
        return Err(
            CheckProjectInfoError::ProjectFileDoesntExists(
                ErrorContext { 
                    project_file_path, 
                } 
            )
        );
    };

    if !project_file_path_buf.is_file() {
        return Err(
            CheckProjectInfoError::SelectedFileIsntFile(
                ErrorContext { 
                    project_file_path, 
                } 
            )
        );
    };

    let mut buf: [u8; PROJECT_FILE_HEADER_SIZE] = [0; PROJECT_FILE_HEADER_SIZE]; 
    
    let mut project_file_handler = TokioFile::open(&project_file_path_buf).await?; 

    project_file_handler.read_exact(&mut buf).await?;

    let project_file_header = ProjectFileHeader::try_from_bytes(&buf)
        .map_err(|error|{
            CheckProjectInfoError::BytemuckPodCastError(error)
        })?;

    if project_file_header.magic != MAGIC_BYTES {
        return Err(
            CheckProjectInfoError::WrongFormat(
                ErrorContext { 
                    project_file_path 
                }
            )
        );
    } 

    let project_id = ProjectID::from_bytes(&project_file_header.project_id);

    Ok(
        CheckProjectInfoContext { 
            project_file_path, 
            project_id,
        }
    )
}

pub fn check_project_info_callback(
    result: Result<CheckProjectInfoContext, CheckProjectInfoError>,
) -> Option<AppEvent> {
    match result {
        Ok(context) => {
            Some(
                OpenProjectEvent::CheckProjectCache { 
                    project_file_path: context.project_file_path, 
                    project_id: context.project_id,
                }.into() 
            ) 
        },
        Err(error) => {
            match error {
                CheckProjectInfoError::STDIOError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::OpenProjectEventError(
                                OpenProjectEventError::STDIOError(error)
                            ) 
                        )
                    )
                }, 
                CheckProjectInfoError::BytemuckPodCastError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::OpenProjectEventError(
                                OpenProjectEventError::BytemuckPodCastError(error)
                            )
                        )
                    )
                },
                CheckProjectInfoError::WrongFormat(context) => {
                    Some(
                        AppEvent::OpenProjectEvent(
                            OpenProjectEvent::WrongFormat { 
                                project_file_path: context.project_file_path 
                            }
                        )
                    )
                },
                CheckProjectInfoError::SelectedFileIsntFile(context) => {
                    Some(
                        AppEvent::OpenProjectEvent(
                            OpenProjectEvent::SelectedFileIsntFile { 
                                project_file_path: context.project_file_path, 
                            }
                        )
                    )
                },
                CheckProjectInfoError::ProjectFileDoesntExists(context) => {
                    Some(
                        AppEvent::OpenProjectEvent(
                            OpenProjectEvent::ProjectFileDoesntExists { 
                                project_file_path: context.project_file_path 
                            }
                        )
                    )
                },
            }
        },
    } 
} 
