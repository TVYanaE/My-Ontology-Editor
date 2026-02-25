use std::{
    path::Path,
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
        event_manager::{
            EventManager,
        },
        job_manager::{
            Job, JobID, JobKind,
        },
        confirmation_cache::{
            ConfirmationContext,
            ConfirmationCache,
        },
    }, 
    logic_core_error::LogicCoreError,
};


pub struct LogicCoreLogic;


impl LogicCoreLogic {
    pub fn check_creating_project_path<S: EventSender>(
        task_id: TaskID,
        project_name: &str,
        project_path: &impl AsRef<Path>,
        event_manager: &EventManager<S>,
        confirmation_cache: &mut ConfirmationCache,
    ) -> Result<Vec<Job>, LogicCoreError<S>> {
        let mut jobs = Vec::with_capacity(2);
        match project_path.as_ref().metadata() {
            Ok(meta) => {
                if !meta.is_dir() {
                    let error_text = format!("Invalid Path: Is not directory");
                    
                    event_manager.send_event(LogicEvent::TaskRespone { 
                        task_id: task_id.clone(), 
                        task_kind: TaskKind::CreateProject { 
                            project_name: project_name.to_string(), 
                            project_path: project_path.as_ref().to_path_buf(), 
                        }, 
                        task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                        }
                    )?;

                    return Ok(jobs);
                }
            },
            Err(error) => {
                let error_text = format!("Invalid Path: {error}");
               
                event_manager.send_event(LogicEvent::TaskRespone { 
                    task_id: task_id.clone(), 
                    task_kind: TaskKind::CreateProject { 
                        project_name: project_name.to_string(), 
                        project_path: project_path.as_ref().to_path_buf(),
                    }, 
                    task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                    }
                )?;

                return Ok(jobs);
            } 
        };

        let mut project_file = project_path.as_ref().to_path_buf();
        project_file.push(project_name);
        project_file.set_extension("vontov");

        if project_file.exists() {
            let confirmation_id = ConfirmationID::new();
            event_manager.send_event(
                LogicEvent::ConfirmationRequested { 
                    confirmation_id: confirmation_id.clone(), 
                    confirmation_kind: ConfirmationKind::Owerrite { 
                        task_id: task_id.clone(),
                        project_name: project_name.to_string(), 
                        project_path: project_path.as_ref().to_path_buf(), 
                    } 
                }
            )?;

            confirmation_cache.push(
                confirmation_id, 
                ConfirmationKind::Owerrite { 
                    task_id, 
                    project_name: project_name.to_string(), 
                    project_path: project_path.as_ref().to_path_buf(), 
                }
            );

            return Ok(jobs)
        }

        jobs.push(
            Job { 
                id: JobID::new(), 
                kind: JobKind::CreateProject { 
                    task_id, 
                    project_name: project_name.to_string(), 
                    project_path: project_path.as_ref().to_path_buf(),
                }
            }
        );

        Ok(jobs)   
    } 

    pub fn create_project<S: EventSender>(
        task_id: TaskID,
        project_name: &str,
        project_path: &impl AsRef<Path>,
        project_manager: &ProjectManager,
        event_manager: &EventManager<S>,
        db_commands: &DBCommands,
    ) -> Result<Vec<Job>, LogicCoreError<S>> { 
        let jobs = Vec::with_capacity(2);

        // Logic for creating project 
        project_manager.create_new_project(
            project_name, 
            project_path,
            db_commands,
        )?;

        event_manager.send_event(
            LogicEvent::TaskRespone { 
                task_id: task_id, 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name.to_string(), 
                    project_path: project_path.as_ref().to_path_buf(), 
                }, 
                task_result: TaskResult::Ok, 
            } 
        )?;

        Ok(jobs) 
    } 

    pub fn confirmation_decline<S: EventSender>(
        event_manager: &EventManager<S>,
        context: ConfirmationContext,
    ) -> Result<Vec<Job>, LogicCoreError<S>> {
        let jobs = Vec::with_capacity(2);
   
        match context {
            ConfirmationContext::CreateProjectContext { 
                task_id, 
                project_name, 
                project_path 
            } => {
                event_manager.send_event(
                    LogicEvent::TaskRespone { 
                        task_id: task_id, 
                        task_kind: TaskKind::CreateProject { 
                            project_name: project_name, 
                            project_path: project_path, 
                        }, 
                        task_result: TaskResult::CanceledByUser,
                    }
                )?;
            },
        }

        Ok(jobs)
    }

    pub fn shutdown(
        db_module_handler: &mut DBModuleHandler 
    ) -> Vec<Job> {
        let jobs = Vec::with_capacity(2);
        match db_module_handler.db_commands.send(DBCommand::Shutdown) {
            Ok(_) => {
                if let Some(handle) = db_module_handler.thread_handle.take() {
                    match handle.join() {
                        Ok(_) => {
                        }, 
                        Err(error) => {
                            error!(error = ?error, "Data Base Thread Panic");                
                        },
                    }
                }
            },
            Err(error) => { 
                error!(error = ?error, "Data Base Thread Panic");                
            },
        };
        jobs 
    }
}


