use std::{
    path::{
        Path,
    },
    fs::{
        File,
    },
};
use super::{
    super::{
        super::{
            events::{
                EventSender, LogicEvent,
                TaskID, TaskKind, TaskResult,
            },
            job_manager::{
                Job,
            },
            event_manager::EventManager,
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
        project_path: &impl AsRef<Path>,
        project_manager: &ProjectManager,
        project_cache: &mut ProjectCache, 
        event_manager: &EventManager<S>,
    ) -> Result<Vec<Job>, LogicCoreError<S>> {
        let jobs = Vec::with_capacity(2);
       
        /*
        if project_path.as_ref().is_dir() {
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
        }

        let project_file = File::open(pro) */ 

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
