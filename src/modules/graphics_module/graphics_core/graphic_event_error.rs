use thiserror::{
    Error
};
use crate::{
    modules::{
        graphics_module::{
            graphics_backend::{
                wgpu_backend::WGPUBackendError,
                egui_backend::EGUIBackendError,
            },         
            ui::UIError,
        },
        logic_module::LogicEvent,
    },
};

#[derive(Debug, Error)]
pub enum GraphicsEventError {
    #[error("WGPU Backend Error {0}")]
    WGPUBackendError(#[from] WGPUBackendError), 

    #[error("MPSC Channel Error {0}")]
    MPSCChannelError(#[from] std::sync::mpsc::SendError<LogicEvent>),

    #[error("EGUI Backeend Error {0}")]
    EGUIBackendError(#[from] EGUIBackendError),

    #[error("Surface Error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError), 

    #[error("UI Error: {0}")]
    UIError(#[from] UIError),
}

