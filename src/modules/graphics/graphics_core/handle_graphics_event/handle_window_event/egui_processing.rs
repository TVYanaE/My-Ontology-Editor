use winit::{
    event::WindowEvent,
};
use super::{
    WindowEventError,
};
use crate::{
    aliases::{
        EGUIEventRespone,
    },
    modules::{
        graphics::{
            graphics_data::GraphicsData,
        },
    },
};

pub struct EGUIProcessingContext<'c> {
    pub graphics_data: &'c mut GraphicsData,
}

/// Return True if event has been consumed by egui
pub fn egui_processing(
    event: &WindowEvent,
    egui_processing_context: EGUIProcessingContext,
) -> Result<EGUIEventRespone, WindowEventError> {
    let egui_data = egui_processing_context
        .graphics_data
        .graphics_backend_data
        .egui_data
        .as_mut()
        .ok_or_else(||{
            WindowEventError::EGUIDataWasntFound
        })?;
    let wgpu_data = egui_processing_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_ref()
        .ok_or_else(||{
            WindowEventError::WGPUDataWasntFound
        })?;

    let egui_response = egui_data
        .egui_winit_state.on_window_event(
            &wgpu_data.window, 
            &event
        ); 

    Ok(egui_response)
}
