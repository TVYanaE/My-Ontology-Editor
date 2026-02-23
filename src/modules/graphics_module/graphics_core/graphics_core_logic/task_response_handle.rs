use std::{
    path::{Path},
};
use crate::{
    modules::{
        graphics_module::{
            graphics_core::{
                graphic_core_error::{
                    GraphicsCoreError,
                },
                GraphicsCoreState,
            },
            ui::{
                UI, UIInputEvent, ChosedModalWindow,
            },
        },
        logic_module::{
            events::{
                TaskResult, TaskError,
            },
        },
    },
};

pub struct TaskResponseHandle;


impl TaskResponseHandle {
    pub fn creating_project_task(
        done_task_result: TaskResult,
        project_path: &impl AsRef<Path>, 
        project_name: &str,
        ui: &mut UI,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        match done_task_result {
            TaskResult::Ok => {
                ui.on_event(UIInputEvent::ShowMainUI)?;

                Ok(Some(GraphicsCoreState::Runnig))
            },
            TaskResult::CanceledByUser => {
                let project_path_str = project_path
                    .as_ref()
                    .to_str()
                    .unwrap_or_default()
                    .to_string();

                    ui.on_event(
                        UIInputEvent::ShowModalWindow(
                            ChosedModalWindow::CreateNewProject { 
                                project_name: Some(project_name.to_string()), 
                                project_path: Some(project_path_str) 
                            }
                        )
                    )?; 
                       
                    Ok(Some(GraphicsCoreState::Runnig))
            },
            TaskResult::Error(error_kind) => {
                match error_kind {
                    TaskError::PathError(err_text) => {
                        let project_path_str = project_path
                            .as_ref()
                            .to_str()
                            .unwrap_or_default()
                            .to_string();

                        ui.on_event(
                            UIInputEvent::ShowModalWindow(
                                ChosedModalWindow::CreateNewProject { 
                                    project_name: Some(project_name.to_string()), 
                                    project_path: Some(project_path_str) 
                                }
                            )
                        )?; 
                        ui.on_event(
                            UIInputEvent::ShowModalWindow(
                                ChosedModalWindow::Notification { 
                                    text: err_text 
                                }
                            )
                        )?;
                         
                        Ok(Some(GraphicsCoreState::Runnig))
                    },
                }
            },
        }
    }  
}
