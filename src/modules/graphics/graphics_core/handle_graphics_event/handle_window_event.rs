mod egui_processing;
mod redraw_handle;
mod resize_handle;

use thiserror::{
    Error
};
use tracing::{
    instrument
};
use winit::{
    event::WindowEvent
};
use crate::{
    modules::{
        graphics::{
            events::{
                EventBuffers,
            },
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
            graphics_core::GraphicsCoreState,
        },
    },
};
use self::{
    egui_processing::{
        egui_processing, EGUIProcessingContext,
    },
    redraw_handle::{
        redraw_handle, RedrawHandleContext,
    },
    resize_handle::{
        resize_handle, ResizeHandleContext,
    },
};

pub struct HandleWindowEventContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
}

#[instrument(skip_all, err)]
pub fn handle_window_event(
    event: WindowEvent,
    handle_window_event_context: HandleWindowEventContext,
) -> Result<Option<GraphicsCoreState>, WindowEventError> {
    let egui_response = egui_processing(
        &event, 
        EGUIProcessingContext { 
            graphics_data: handle_window_event_context.graphics_data 
        },
    )?;   
 
    if egui_response.repaint {
        redraw_handle(
            RedrawHandleContext { 
                event_buffers: handle_window_event_context.event_buffers, 
                graphics_data: handle_window_event_context.graphics_data, 
                graphics_states: handle_window_event_context.graphics_states 
            }
        )?; 
    } 

    if egui_response.consumed {
        return Ok(None);
    }

    let new_graphic_state = match event {
        WindowEvent::CloseRequested => Some(GraphicsCoreState::Shutdown),
        WindowEvent::Resized(physical_size) => {
            resize_handle(
                physical_size, 
                ResizeHandleContext { 
                    graphics_data: handle_window_event_context.graphics_data 
                }
            )?;

            None
        }, 
        _ => None
    }; 

    return Ok(new_graphic_state)
}

#[derive(Debug, Error)]
pub enum WindowEventError {
    #[error("WGPU Data wasn't found")]
    WGPUDataWasntFound,

    #[error("EGUI Data wasn't found")]
    EGUIDataWasntFound,

    #[error("Get Current Surface Texture Error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),

}
