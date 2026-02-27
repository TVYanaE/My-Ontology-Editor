use std::{
    path::PathBuf,
};
use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        logic_module::{
            prelude::{
                logic_command::LogicCommand,
                TaskID, TaskKind, LogicModuleHandler, 
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
            ui::{UI, UIInputEvent, ChosedModalWindow},
        },  
    },
};

impl GraphicsCoreLogic {
    pub fn create_project(
        logic_module_handler: &mut LogicModuleHandler,
        ui: &mut UI,
        task_cache: &mut TaskCache,
        project_name: String,
        project_path: PathBuf,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        let task_id = TaskID::new(); 
        logic_module_handler.logic_commands.send(
            LogicCommand::Task { 
                task_id: task_id.clone(), 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name.clone(), 
                    project_path: project_path.clone(), 
                } 
            }
        )?;

        ui.on_event(
            UIInputEvent::ShowModalWindow(
                ChosedModalWindow::WaitingWindow { 
                    text: "Creating Project. Please Wait".into() 
                }
            )
        )?; 
        
        task_cache.push(
            task_id.clone(), 
            TaskContext::CreateProjectContext { 
                project_name, 
                project_path 
            }
        );
        

        Ok(Some(GraphicsCoreState::WaitingTask {
            task_id: task_id, 
        }))
    } 
}
