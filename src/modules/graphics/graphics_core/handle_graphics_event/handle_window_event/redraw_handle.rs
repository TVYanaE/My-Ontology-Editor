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
) {
    let full_output = prepare_phase(
        PreparePhaseContext { 
            graphics_data: redraw_handle_context.graphics_data, 
            event_buffers: redraw_handle_context.event_buffers,
            ui_state: &mut redraw_handle_context.graphics_states.ui_state
        }
    );
    draw_phase(
        full_output, 
        DrawPhaseContext { 
            graphics_data: redraw_handle_context.graphics_data 
        }
    );
}
