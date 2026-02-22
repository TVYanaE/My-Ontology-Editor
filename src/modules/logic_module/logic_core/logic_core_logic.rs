use std::{
    path::PathBuf,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
    },
};
use super::{
    super::{
        events::{
            LogicEvent, TaskError,
            EventSender, TaskKind,
            TaskResult, TaskID,
        }, 
    },
    LogicCoreState,
    LogicCoreError
};


pub struct LogicCoreLogic;

#[derive(Debug)]
pub enum WorkAfterConfirmation {
    CreateProject
}

pub struct CreateProjectContext<'c, S: EventSender> {
    pub app_dirs: &'c ApplicationDirectories,
    pub project_name: String,
    pub project_path: PathBuf,
    pub task_id: TaskID, 
    pub event_sender: &'c S,
}


impl LogicCoreLogic {
    pub fn create_project_handle<S: EventSender>(
    context: CreateProjectContext<S>,
) -> Result<Option<LogicCoreState>, LogicCoreError<S>> { 
    match context.project_path.metadata() {
        Ok(meta) => {
            if !meta.is_dir() {
                let error_text = format!("Invalid Path: Is not directory");
                
                context.event_sender.send_event(LogicEvent::TaskRespone { 
                    task_id: context.task_id, 
                    task_kind: TaskKind::CreateProject { 
                        project_name: context.project_name, 
                        project_path: context.project_path 
                    }, 
                    task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                    }
                ).map_err(|error|
                    LogicCoreError::EventSenderError(error)
                )?;

                return Ok(None);
            }
        },
        Err(error) => {
            let error_text = format!("Invalid Path: {error}");
           
            context.event_sender.send_event(LogicEvent::TaskRespone { 
                task_id: context.task_id, 
                task_kind: TaskKind::CreateProject { 
                    project_name: context.project_name, 
                    project_path: context.project_path 
                }, 
                task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                }
            ).map_err(|error|
                LogicCoreError::EventSenderError(error)
            )?;

            return Ok(None);
        }
    }

    /* context.project_manager.create_project(
        CreateProjectDescriptor { 
            project_name: context.project_name, 
            project_dir: context.project_dir, 
            projects_dir_cache_path: context.app_dirs.cache_directory.projects_dir.clone(),
        },
        context.custom_events
    )?; */

    context.event_sender.send_event(
        LogicEvent::TaskRespone { task_id: context.task_id, 
            task_kind: TaskKind::CreateProject { 
                project_name: context.project_name, 
                project_path: context.project_path 
            }, 
            task_result: TaskResult::Ok, 
        } 
    ).map_err(|error|
        LogicCoreError::EventSenderError(error)
    )?;

    Ok(None)
}
}


