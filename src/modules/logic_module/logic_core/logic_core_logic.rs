use std::{
    path::{PathBuf, Path},
};
use tracing::{
    error,
};
use crate::{
    modules::{
        db_module::{
            DBModuleHandler, DBCommand,
            DBCommands,
        },
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
        project_manager::ProjectManager,
    }, 
    logic_core_error::LogicCoreError,
    LogicCoreState, 
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

impl LogicCoreLogic {
    pub fn check_creating_project_path<S: EventSender>(
        task_id: &TaskID,
        project_name: &str,
        project_path: &impl AsRef<Path>,
        event_sender: &S,
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> {
        match project_path.as_ref().metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    let error_text = format!("Invalid Path: Is not directory");
                    
                    event_sender.send_event(LogicEvent::TaskRespone { 
                        task_id: task_id.clone(), 
                        task_kind: TaskKind::CreateProject { 
                            project_name: project_name.to_string(), 
                            project_path: project_path.as_ref().to_path_buf(), 
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
               
                event_sender.send_event(LogicEvent::TaskRespone { 
                    task_id: task_id.clone(), 
                    task_kind: TaskKind::CreateProject { 
                        project_name: project_name.to_string(), 
                        project_path: project_path.as_ref().to_path_buf(),
                    }, 
                    task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                    }
                ).map_err(|error|
                    LogicCoreError::EventSenderError(error)
                )?;

                return Ok(None);
            } 
        };

        let mut project_file = project_path.as_ref().to_path_buf();
        project_file.push(project_name);
        project_file.set_extension("vontov");

        if project_file.exists() {
            let confirmation_id = ConfirmationID::new();
            event_sender.send_event(
                LogicEvent::ConfirmationRequested { 
                    confirmation_id: confirmation_id.clone(), 
                    confirmation_kind: ConfirmationKind::Owerrite { 
                        project_name: project_name.to_string(), 
                        project_path: project_path.as_ref().to_path_buf(), 
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
                            task_id: task_id.clone(),
                            project_name: project_name.to_string(),
                            project_path: project_path.as_ref().to_path_buf(),
                        }, 
                    }
                )
            );
        }

        Ok(None)
    } 

    pub fn create_project<S: EventSender>(
        task_id: &TaskID,
        owerrite_confirmation: Option<bool>,
        project_name: &str,
        project_path: &impl AsRef<Path>,
        project_manager: &ProjectManager,
        event_sender: &S,
        db_commands: &DBCommands,
    ) -> Result<Option<LogicCoreState>, LogicCoreError<S>> { 
        // Check the confirmation if it is 
        if let Some(owerrite) = owerrite_confirmation {
            if !owerrite {
                event_sender.send_event(
                LogicEvent::TaskRespone { 
                    task_id: task_id.clone(), 
                    task_kind: TaskKind::CreateProject { 
                        project_name: project_name.to_string(), 
                        project_path: project_path.as_ref().to_path_buf(), 
                    }, 
                    task_result: TaskResult::CanceledByUser, 
                } 
                ).map_err(|error|
                    LogicCoreError::EventSenderError(error)
                )?;        
                return Ok(Some(LogicCoreState::Ready));
            }
        }

        // Logic for creating project 
        project_manager.create_new_project(
            project_name, 
            project_path,
            db_commands,
        )?;

        event_sender.send_event(
            LogicEvent::TaskRespone { 
                task_id: task_id.clone(), 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name.to_string(), 
                    project_path: project_path.as_ref().to_path_buf(), 
                }, 
                task_result: TaskResult::Ok, 
            } 
        ).map_err(|error|
            LogicCoreError::EventSenderError(error)
        )?;

        Ok(None)    
    } 

    pub fn shutdown(
        db_module_handler: &mut DBModuleHandler 
    ) -> Option<LogicCoreState> {
        match db_module_handler.db_commands.send(DBCommand::Shutdown) {
            Ok(_) => {
                if let Some(handle) = db_module_handler.thread_handle.take() {
                    match handle.join() {
                        Ok(_) => {
                            Some(LogicCoreState::Shutdown)
                        }, 
                        Err(error) => {
                            error!(error = ?error, "Data Base Thread Panic");                
                            Some(LogicCoreState::Shutdown)
                        },
                    }
                }
                else {
                    Some(LogicCoreState::Shutdown)
                }
            },
            Err(error) => { 
                error!(error = ?error, "Data Base Thread Panic");                
                Some(LogicCoreState::Shutdown)
            },
        }
    }
}


