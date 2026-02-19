mod draw_phase; 

use winit::{
    event::WindowEvent,
};
use thiserror::{
    Error
};
use tracing::{
    instrument,
    error,
};
use crate::{
    modules::{
        logic_module::{
            LogicEvent,
        },
        graphics_module::{
            ui::UIInputEvent,
        }, 
    },
};
use super::{
    super::{
        ui::UI,
        LogicModuleHandler,  
        graphics_backend::{
            wgpu_backend::WGPUBackendError,
            egui_backend::EGUIBackendError,
            GraphicsBackend,
        },
    },
    graphics_event::{
        InternalEvent,
        ExternalEvent,
    },
    CustomEvents,
    GraphicsCoreState,
};
use self::{
    draw_phase::draw_phase
};

pub struct GraphicsCoreLogic;

#[derive(Debug, Error)]
pub enum InternalEventError {
    #[error("WGPU Backend Error {0}")]
    WGPUBackendError(#[from] WGPUBackendError), 
    
    #[error("MPSC Channel Error {0}")]
    MPSCChannelError(#[from] std::sync::mpsc::SendError<LogicEvent>),
}

#[derive(Debug, Error)]
pub enum ExternalEventError {
    
}

#[derive(Debug, Error)]
pub enum RedrawEventError {
    #[error("WGPU Backend Error {0}")]
    WGPUBackendError(#[from] WGPUBackendError),

    #[error("EGUI Backeend Error {0}")]
    EGUIBackendError(#[from] EGUIBackendError),

    #[error("Surface Error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
}

#[derive(Debug, Error)]
pub enum WindowEventError {
    #[error("WGPU Backend Error {0}")]
    WGPUBackendError(#[from] WGPUBackendError), 

    #[error("EGUI Backeend Error {0}")]
    EGUIBackendError(#[from] EGUIBackendError),
}

impl GraphicsCoreLogic {
    #[instrument(skip_all,err)]
    pub fn internal_event_handle(
        event: InternalEvent,
        graphics_backend: &mut GraphicsBackend,
        logic_module_handler: &mut LogicModuleHandler,
        ui: &mut UI,
    ) -> Result<Option<GraphicsCoreState>, InternalEventError> {
        match event {
            InternalEvent::AppShutdownReq => {
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
            },
            InternalEvent::ResumedEvent(window) => {
                graphics_backend.wgpu_backend.init(window)?;
                let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
                graphics_backend.egui_backend.init(wgpu_data);

                Ok(None)
            }
            InternalEvent::CreateProjectReq{project_name, project_dir} => {
                logic_module_handler.sender.send(LogicEvent::CreateProject{ 
                    project_name: project_name, 
                    project_dir: project_dir 
                })?;

                ui.on_event(UIInputEvent::Waiting);

                Ok(Some(GraphicsCoreState::Waiting))
            }
        }
    }       

    #[instrument(skip_all,err)]
    pub fn external_event_handle(
        event: ExternalEvent,
        ui: &mut UI
    ) -> Result<Option<GraphicsCoreState>, ExternalEventError> {
        match event {
            ExternalEvent::TaskDone => {
                ui.on_event(UIInputEvent::StopWaiting);
                Ok(Some(GraphicsCoreState::Runnig))
            }
            ExternalEvent::AppShutdownReq => {
                Ok(Some(GraphicsCoreState::Shutdown))
            }
        }
    }

    #[instrument(skip_all,err)]
    pub fn window_event_handle(
        event: WindowEvent,
        graphics_backend: &mut GraphicsBackend,  
    ) -> Result<Option<GraphicsCoreState>, WindowEventError> {
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        let resp = graphics_backend.egui_backend.on_window_event(&event, wgpu_data)?;

        if resp.repaint {
            wgpu_data.window.request_redraw();
        }

        if resp.consumed {
            return Ok(None);
        }

        match event {
            WindowEvent::Resized(physical_size) => {
                graphics_backend.wgpu_backend.resize(physical_size)?; 
                Ok(None)
            }
            _ => {
                Ok(None)
            }
        }
    }

    #[instrument(skip_all,err)]
    pub fn redraw_event_handle(
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
        custom_events: &CustomEvents,
    ) -> Result<(), RedrawEventError> {
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        let full_output = graphics_backend.egui_backend.prepare_ui(
            wgpu_data, 
            ui, 
            custom_events
        )?;
        let egui_data = graphics_backend.egui_backend.get_mut_egui_data()?;
        draw_phase(full_output, wgpu_data, egui_data)?;

        Ok(())
    }
}
