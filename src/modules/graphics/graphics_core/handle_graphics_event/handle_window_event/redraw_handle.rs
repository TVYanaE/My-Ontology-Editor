mod draw_phase;
mod prepare_phase;

use crate::{
    modules::{
        graphics::{
            events::EventBuffers,
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
        },
    },
};
use super::{
    WindowEventError,
};
use self::{
    draw_phase::{
        draw_phase, DrawPhaseContext,
    },
    prepare_phase::{
        prepare_phase, PreparePhaseContext,
    },
};

pub struct RedrawHandleContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub graphics_data: &'c mut GraphicsData,
    pub graphics_states: &'c mut GraphicsStates,
}

pub fn redraw_handle(
    redraw_handle_context: RedrawHandleContext,
) -> Result<(), WindowEventError> {
    let wgpu_data = redraw_handle_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_ref()
        .ok_or_else(||{
            WindowEventError::WGPUDataWasntFound
        })?;
    let egui_data = redraw_handle_context
        .graphics_data
        .graphics_backend_data
        .egui_data
        .as_mut()
        .ok_or_else(||{
            WindowEventError::EGUIDataWasntFound
        })?;     
        

    let full_output = prepare_phase(
        PreparePhaseContext { 
            egui_data: egui_data,
            wgpu_data: wgpu_data,
            ui_data: &mut redraw_handle_context.graphics_data.ui_data,
            event_buffers: redraw_handle_context.event_buffers,
            ui_state: &mut redraw_handle_context.graphics_states.ui_state
        }
    );
    draw_phase(
        full_output, 
        DrawPhaseContext { 
            wgpu_data: wgpu_data,
            egui_data: egui_data,
        }
    )?;
    Ok(())
}
