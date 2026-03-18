use std::path::PathBuf;

use thiserror::Error;

use super::super::super::super::app_event::AppEvent;
use super::super::super::super::app_event::creating_project_event::CreatingProjectEvent;

#[derive(Debug, Error)]
pub enum CheckProjectInfoError {
    #[error("Path isn't exist")]
    ProjectDirIsntExsist(CheckProjectInfoContext),

    #[error("Path isn't dir")]
    ProjectDirPathIsntDir(CheckProjectInfoContext),

    #[error("Project File already exist")]
    ProjectFileAlreadyExist(CheckProjectInfoContext),
}

#[derive(Debug)]
pub struct CheckProjectInfoContext {
    project_name: String,
    project_path: String,
}

pub fn check_project_info(
    project_name: String, 
    project_path: String,
) -> Result<CheckProjectInfoContext, CheckProjectInfoError> {
    let mut project_path_buf = PathBuf::new();

    project_path_buf.push(&project_path);

    if !project_path_buf.exists() {
        return Err(
            CheckProjectInfoError::ProjectDirIsntExsist(
                CheckProjectInfoContext { 
                    project_name: project_name, 
                    project_path: project_path, 
                }
            )
        );
    };

    if !project_path_buf.is_dir() {
        return Err(
            CheckProjectInfoError::ProjectDirPathIsntDir(
                CheckProjectInfoContext { 
                    project_name: project_name, 
                    project_path: project_path, 
                }
            )
        );
    };

    project_path_buf.push(&project_name);
    project_path_buf.set_extension("vontov");

    if project_path_buf.exists() {
        return Err(
            CheckProjectInfoError::ProjectFileAlreadyExist(
                CheckProjectInfoContext { 
                    project_name: project_name, 
                    project_path: project_path, 
                }
            )
        );
    }

    Ok(CheckProjectInfoContext { project_name, project_path })
} 

pub fn check_project_info_callback(
    result: Box<Result<CheckProjectInfoContext, CheckProjectInfoError>>
) -> Option<AppEvent> {
    match *result {
        Ok(context) => {
            Some(
                CreatingProjectEvent::CreateProject { 
                    project_name: context.project_name, 
                    project_path: context.project_path, 
                }.into()
            )
        },
        Err(error) => {
            match error {
                CheckProjectInfoError::ProjectDirIsntExsist(context) => {
                    Some(
                        CreatingProjectEvent::ProjectDirIsntExsist { 
                            project_name: context.project_name, 
                            project_path: context.project_path, 
                        }.into()
                    )
                },
                CheckProjectInfoError::ProjectDirPathIsntDir(context) => {
                    Some(
                        CreatingProjectEvent::ProjectDirPathIsntDir { 
                            project_name: context.project_name, 
                            project_path: context.project_path, 
                        }.into()
                    )
                },
                CheckProjectInfoError::ProjectFileAlreadyExist(context) => {
                    Some(
                        CreatingProjectEvent::ProjectFileAlreadyExist { 
                            project_name: context.project_name, 
                            project_path: context.project_path, 
                        }.into()
                    )
                },
            }
        },
    }
}
