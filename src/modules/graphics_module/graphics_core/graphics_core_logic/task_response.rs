mod creating_project_task;

use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        logic_module::{
            prelude::{
                TaskID, TaskResult
            }, 
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            task_cache::{
                TaskCache, TaskContext,
            }, 
            ui::UI,
        },  
    },
};

struct TaskResponse;

impl GraphicsCoreLogic {
    pub fn task_response(
        waiting_task_id: TaskID,
        done_task_id: TaskID,
        done_task_result: TaskResult,
        ui: &mut UI,
        task_cache: &mut TaskCache,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> 
    { 
        if waiting_task_id == done_task_id {
            // TODO: Logic for task cache problem connected with Task Context {}
            if let Some(context) = task_cache.remove(&done_task_id) {
                match context {
                    TaskContext::CreateProjectContext { 
                        project_name, 
                        project_path 
                    } => {
                        let new_state = TaskResponse::creating_project_task(
                            done_task_result, 
                            &project_path, 
                            &project_name, 
                            ui
                        )?;

                        Ok(new_state)
                    },
                }
            }
            else {
                Err(GraphicsCoreError::TaskContextNotFound)
            }
        }
        else {       
            Ok(None) 
        }
    } 
}
