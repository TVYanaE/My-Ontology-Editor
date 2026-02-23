mod draw_phase; 
mod task_response_handle;

use std::{
    path::PathBuf,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
};
use tracing::{
    error,
};
use crate::{
    modules::{
        logic_module::{
            events::{
                LogicCommand, 
                TaskID, TaskKind,
                TaskResult,
                DecisionKind, 
                ConfirmationID, ConfirmationKind,
            }, 
            logic_module_handler::LogicModuleHandler,
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            graphics_backend::{
                GraphicsBackend,
            },
            ui::{UI, UIInputEvent, ChosedModalWindow},
            events::{
                CustomEvents
            },
        },  
    },
};
use self::{
    draw_phase::draw_phase, 
    task_response_handle::TaskResponseHandle,
};

pub struct GraphicsCoreLogic;


impl GraphicsCoreLogic {
    pub fn resumed_event_handle(
        graphics_backend: &mut GraphicsBackend,
        window: Window,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        graphics_backend.wgpu_backend.init(window)?;
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        graphics_backend.egui_backend.init(wgpu_data);

        Ok(None) 
    }

    pub fn app_shutdown_handle(
        logic_module_handler: &mut LogicModuleHandler,
    ) -> Option<GraphicsCoreState> {
        // logic for shutdown
        match logic_module_handler.logic_commands.send(LogicCommand::Shutdown) {
            Ok(_) => {
                if let Some(handle) = logic_module_handler.thread_handle.take() {
                    // Error will come due to panic in thread 
                    match handle.join() {
                        Ok(_) => {
                            Some(GraphicsCoreState::Shutdown)
                        },
                        Err(error) => {
                            error!(error = ?error, "Logic Thread Panic");                
                            Some(GraphicsCoreState::Shutdown)
                        }
                    }
                }
                else { 
                    Some(GraphicsCoreState::Shutdown)
                }
            },
            Err(error) => { 
                error!(error = ?error, "Logic Thread Panic");                
                Some(GraphicsCoreState::Shutdown)
            },
        } 
    }

    pub fn create_project_req_handle(
        logic_module_handler: &mut LogicModuleHandler,
        ui: &mut UI,
        project_name: String,
        project_path: PathBuf,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        let task_id = TaskID::new(); 
        logic_module_handler.logic_commands.send(
            LogicCommand::Task { 
                task_id: task_id.clone(), 
                task_kind: TaskKind::CreateProject { 
                    project_name: project_name, 
                    project_path: project_path 
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

        Ok(Some(GraphicsCoreState::WaitingTask {
            task_id: task_id, 
        }))
    }
       
    pub fn task_response_handle(
        waiting_task_id: TaskID,
        done_task_id: TaskID,
        done_task_kind: TaskKind,
        done_task_result: TaskResult,
        ui: &mut UI,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> 
    {
        if waiting_task_id == done_task_id {
            match done_task_kind {
                TaskKind::CreateProject { 
                    project_name, 
                    project_path 
                } => {
                    let new_state = TaskResponseHandle::creating_project_task(
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
            Ok(None) 
        }
    } 

    pub fn confirmation_required_handle(
        ui: &mut UI,
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        match confirmation_kind.clone() {
            ConfirmationKind::Owerrite { project_name, .. } => {
                let confirmation_text = format!("Project {} already extists. Replace?", project_name);

                ui.on_event(
                    UIInputEvent::ShowModalWindow(
                        ChosedModalWindow::ConfirmationWindow { 
                            confirmation_id, 
                            confirmation_kind, 
                            text: confirmation_text 
                        }
                    )
                )?; 
                Ok(None)
            },
        }
    } 

    pub fn confirmation_obtain_handle(
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind,
        logic_module_handler: &mut LogicModuleHandler,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        logic_module_handler.logic_commands.send(
            LogicCommand::ConfirmationDecision { 
                confirmation_id: confirmation_id, 
                decision: decision, 
                decision_kind: decision_kind, 
            }
        )?;

        Ok(None)
    }

    pub fn resize_handle(
        physical_size: PhysicalSize<u32>, 
        graphics_backend: &mut GraphicsBackend,  
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
         
        graphics_backend.wgpu_backend.resize(physical_size)?; 
        Ok(None)
    } 

    pub fn redraw_event_handle(
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
        custom_events: &CustomEvents,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        let full_output = graphics_backend.egui_backend.prepare_ui(
            wgpu_data, 
            ui, 
            custom_events
        )?;
        let egui_data = graphics_backend.egui_backend.get_mut_egui_data()?;
        draw_phase(full_output, wgpu_data, egui_data)?;

        Ok(None)
    }
}
