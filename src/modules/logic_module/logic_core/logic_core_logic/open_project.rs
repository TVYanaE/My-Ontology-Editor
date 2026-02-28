use std::{
    path::{
        Path,
    }, 
};
use super::{
    super::{
        super::{
            db_core::DBCore,
            logic_module_io::{
                event_sender::EventSender,
                logic_event::LogicEvent,
                event_manager::EventManager,
                TaskID, TaskResult,
                TaskError,
            },
            job_manager::{
                Job,
            },
            project_manager::ProjectManager,
            project_cache::ProjectCache,
        },
        logic_core_error::LogicCoreError,
    },
    LogicCoreLogic,
};

impl LogicCoreLogic {
    pub fn open_project<S: EventSender>(
        task_id: TaskID,
        project_file_path: &impl AsRef<Path>,
        project_manager: &ProjectManager,
        project_cache: &mut ProjectCache, 
        event_manager: &EventManager<S>,
        db_core: &mut DBCore,
    ) -> Result<Vec<Job>, LogicCoreError<S>> {
        let jobs = Vec::with_capacity(2);
       
        
        if project_file_path.as_ref().is_dir() {
            event_manager.send_event(
                LogicEvent::TaskRespone { 
                    task_id: task_id.clone(), 
                    task_result: TaskResult::Error(
                        TaskError::PathError("
                        Choosed File is not file. Please choose project file
                    ".to_string())
                    ), 
                } 
            )?;
        }

        if !project_file_path.as_ref().exists() {
            event_manager.send_event(
                LogicEvent::TaskRespone { 
                    task_id: task_id.clone(), 
                    task_result: TaskResult::Error(
                        TaskError::PathError("
                        Choosed File is not exists
                    ".to_string())
                    ), 
                } 
            )?;
        }


        // TODO Send event to UI Module
        /* event_manager.send_event(
            LogicEvent::TaskRespone { 
                task_id: task_id, 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name.to_string(), 
                    project_path: project_path.as_ref().to_path_buf(), 
                }, 
                task_result: TaskResult::Ok, 
            } 
        )?; */

        return Ok(jobs)
    }
}
