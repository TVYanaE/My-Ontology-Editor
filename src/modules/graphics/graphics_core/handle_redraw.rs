mod draw_phase;
mod prepare_phase;

use thiserror::{Error};
use tracing::{instrument};
use crate::{
    modules::{
        graphics::{
            events::EventBuffers,
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
        },
    },
};
use self::{
    draw_phase::{
        draw_phase, DrawPhaseContext,
    },
    prepare_phase::{
        prepare_phase, PreparePhaseContext,
    },
};

pub struct HandleRedrawContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub graphics_data: &'c mut GraphicsData,
    pub graphics_states: &'c mut GraphicsStates,
}

#[instrument(skip_all, err)]
pub fn handle_redraw(
    handle_redraw_context: HandleRedrawContext,
) -> Result<(), RedrawError> {
    let wgpu_data = handle_redraw_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_ref()
        .ok_or_else(||{
            RedrawError::WGPUDataWasntFound
        })?;
    let egui_data = handle_redraw_context
        .graphics_data
        .graphics_backend_data
        .egui_data
        .as_mut()
        .ok_or_else(||{
            RedrawError::EGUIDataWasntFound
        })?;     
        

    let full_output = prepare_phase(
        PreparePhaseContext { 
            egui_data: egui_data,
            wgpu_data: wgpu_data,
            ui_data: &mut handle_redraw_context.graphics_data.ui_data,
            event_buffers: handle_redraw_context.event_buffers,
            ui_state: &mut handle_redraw_context.graphics_states.ui_state
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

#[derive(Debug, Error)]
pub enum RedrawError {
    #[error("WGPU Data wasn't found")]
    WGPUDataWasntFound,

    #[error("EGUI Data wasn't found")]
    EGUIDataWasntFound,

    #[error("Get Current Surface Texture Error: {0}")]
    SurfaceError(#[from] wgpu::SurfaceError),
}
