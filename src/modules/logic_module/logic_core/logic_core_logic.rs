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
            ConfirmationKind, ConfirmationID,
        }, 
    },
    LogicCoreState,
    LogicCoreError
};


pub struct LogicCoreLogic;

#[derive(Debug)]
pub enum WorkAfterConfirmation {
    CreateProject {
        task_id: TaskID,
        project_name: String,
        project_path: PathBuf,
    }
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
        };

        let mut project_name = context.project_path.clone();
        project_name.push(&context.project_name);

        if project_name.exists() {
            let confirmation_id = ConfirmationID::new();
            context.event_sender.send_event(
                LogicEvent::ConfirmationRequested { 
                    confirmation_id: confirmation_id.clone(), 
                    confirmation_kind: ConfirmationKind::Owerrite { 
                        project_name: context.project_name.clone(), 
                        project_path: context.project_path.clone(), 
                    } 
                }
            ).map_err(|error|
                LogicCoreError::EventSenderError(error)
            )?;

            return Ok(
                Some(
                    LogicCoreState::WaitConfirmation { 
                        confirmation_id, 
                        work_after_confirmation: WorkAfterConfirmation::CreateProject {
                            task_id: context.task_id.clone(),
                            project_name: context.project_name.clone(),
                            project_path: context.project_path.clone(),
                        }, 
                    }
                )
            );
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

    pub fn for_test<S: EventSender>(
        task_id: TaskID,
        project_name: String,
        project_path: PathBuf,
        decision: bool,
        event_sender: &S
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>>{
        if decision {
            println!("Decision Yes");
        }
        else {
            println!("Decision No");
        }

        event_sender.send_event(
            LogicEvent::TaskRespone { 
                task_id: task_id, 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name, 
                    project_path: project_path 
                }, 
                task_result: TaskResult::Ok, 
            } 
        ).map_err(|error|
            LogicCoreError::EventSenderError(error)
        )?;

        Ok(Some(LogicCoreState::Ready))
    }
}


