mod draw_phase; 

use std::{
    path::PathBuf,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
};
use uuid::{
    Uuid
};
use tracing::{
    error,
};
use crate::{
    modules::{
        logic_module::{
            LogicEvent,
        },
        graphics_module::{
            graphics_core::{
                graphic_event_error::GraphicsEventError,
                GraphicsCoreState,
                pending_task::PendingTask,
            },
            graphics_backend::{
                GraphicsBackend,
            },
            ui::{UI, UIInputEvent},
            events::{
                CustomEvents
            },
        }, 
        shared::{
            logic_module_handler::LogicModuleHandler,
            task_id::TaskID,
        },
    },
};
use self::{
    draw_phase::draw_phase, 
};

pub struct GraphicsCoreLogic;


impl GraphicsCoreLogic {
    pub fn resumed_event_handle(
        graphics_backend: &mut GraphicsBackend,
        window: Window,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
        graphics_backend.wgpu_backend.init(window)?;
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        graphics_backend.egui_backend.init(wgpu_data);

        Ok(None) 
    }

    pub fn app_shutdown_handle(
        logic_module_handler: &mut LogicModuleHandler
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
        // logic for shutdown
        if let Err(_) = logic_module_handler.sender.send(LogicEvent::Shutdown) {
            return Ok(Some(GraphicsCoreState::Shutdown));
        }

        if let Some(handle) = logic_module_handler.thread_handle.take() {
            // Error will come due to panic in thread 
            if let Err(error) = handle.join() {
                error!(error = ?error, "Logic Thread Panic");                
            }
        }

        Ok(Some(GraphicsCoreState::Shutdown)) 
    }

    pub fn create_project_req_handle(
        logic_module_handler: &mut LogicModuleHandler,
        ui: &mut UI,
        project_name: String,
        project_dir: PathBuf,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
        let task_id = Uuid::new_v4(); 
        logic_module_handler.sender.send(LogicEvent::CreateProject{ 
            task_id: TaskID(task_id),
            project_name: project_name, 
            project_dir: project_dir 
        })?;

        ui.on_event(UIInputEvent::Waiting)?;

        Ok(Some(GraphicsCoreState::WaitingTask {
            task_id: TaskID(task_id), 
            pending_task: PendingTask::CreateProject,
        }))
    }
       
    pub fn pending_task_handle(
        pending_task_id: TaskID,
        pending_task: PendingTask,
        done_task_id: TaskID,
        ui: &mut UI,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> 
    {
        if pending_task_id == done_task_id {
            match pending_task {
                PendingTask::CreateProject => {
                    ui.on_event(UIInputEvent::StopWaiting)?;
                    Ok(Some(GraphicsCoreState::Runnig))
                }
            } 
        }
        else {       
            Ok(None) 
        }
    } 


    pub fn confirmation_required_handle(
        ui: &mut UI,
        task_id: TaskID,
        text: &str,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
        ui.on_event(UIInputEvent::ShowConfirmationWindow { task_id, text: text.to_string() })?; 

        Ok(None)
    } 

    pub fn confirmation_obtain_handle(
        task_id: TaskID,
        confirm: bool,
        logic_module_handler: &mut LogicModuleHandler,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
        logic_module_handler.sender.send(LogicEvent::Confirmation { task_id, confirm })?;

        Ok(None)
    }

    pub fn resize_handle(
        physical_size: PhysicalSize<u32>, 
        graphics_backend: &mut GraphicsBackend,  
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
         
        graphics_backend.wgpu_backend.resize(physical_size)?; 
        Ok(None)
    } 

    pub fn redraw_event_handle(
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
        custom_events: &CustomEvents,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
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
