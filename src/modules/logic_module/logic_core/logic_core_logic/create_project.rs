use std::{
    path::Path,
};
use crate::{
    modules::{
        db_module::{
            DBCommands,
        },
    },
};
use super::{
    super::{
        super::{
            events::{
                LogicEvent,
                EventSender,
                TaskResult, TaskID,
            }, 
            project_manager::ProjectManager,
            event_manager::{
                EventManager,
            },
            job_manager::{
                Job, 
            },
            
        }, 
        logic_core_error::LogicCoreError,
    },
    LogicCoreLogic, 
};

impl LogicCoreLogic {
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
                task_result: TaskResult::Ok, 
            } 
        )?;

        Ok(jobs) 
    } 
}
