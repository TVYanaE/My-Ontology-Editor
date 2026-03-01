use std::{
    path::{
        Path,
    },
};
use super::{
    super::{
        super::{
            infrastructure::{
                logic_module_io::{
                    event_manager::EventManager,
                    event_sender::EventSender,
                    logic_event::LogicEvent,
                    TaskID, TaskResult,
                },
                job_manager::{
                    Job,  
                },
            },
            db_core::DBCore,
            project_manager::ProjectManager,
            project_cache::ProjectCache,
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
        project_cache: &mut ProjectCache,
        event_manager: &EventManager<S>,
        db_core: &mut DBCore,
    ) -> Result<Vec<Job>, LogicCoreError<S>> { 
        let jobs = Vec::with_capacity(2);

        // Logic for creating project 
        let project = project_manager.create_new_project(
            project_name, 
            project_path,
            db_core,
        )?;

        project_cache.push(project.get_project_id(), project);
         
        event_manager.send_event(
            LogicEvent::TaskRespone { 
                task_id: task_id,  
                task_result: TaskResult::Ok, 
            }
        )?; 

        Ok(jobs) 
    } 
}
