use std::{
    path::Path,
};
use super::{
    super::{
        super::{
            events::{
                LogicEvent, TaskError,
                EventSender, TaskKind,
                TaskResult, TaskID,
                ConfirmationKind, ConfirmationID,
            },
            event_manager::{
                EventManager,
            },
            job_manager::{
                Job, JobID, JobKind,
            },
            confirmation_cache::{
                ConfirmationCache,
            },
            project::{
                PROJECT_EXTENSION,
            },
        }, 
        logic_core_error::LogicCoreError,
    }, 
    LogicCoreLogic,
};

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
                    task_result: TaskResult::Error(TaskError::PathError(error_text)) 
                    }
                )?;

                return Ok(jobs);
            } 
        };

        let mut project_file = project_path.as_ref().to_path_buf();
        project_file.push(project_name);
        project_file.set_extension(PROJECT_EXTENSION);

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
}
