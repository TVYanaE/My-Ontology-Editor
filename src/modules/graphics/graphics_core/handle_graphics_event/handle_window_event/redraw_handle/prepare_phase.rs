use egui::{
    FullOutput
};
use crate::{
    modules::{
        graphics::{
            events::EventBuffers,
            graphics_data::GraphicsData,
            graphics_states::ui_state::UIState,
            ui::{ui, UIContext},
        },
    },
};

pub struct PreparePhaseContext<'c> {
    pub graphics_data: &'c mut GraphicsData,
    pub event_buffers: &'c mut EventBuffers,
    pub ui_state: &'c mut UIState,
}

pub fn prepare_phase(
    mut prepare_phase_context: PreparePhaseContext,
) -> FullOutput {
    let wgpu_data = prepare_phase_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_ref()
        .unwrap();

    let egui_data = prepare_phase_context
        .graphics_data
        .graphics_backend_data
        .egui_data
        .as_mut()
        .unwrap();

    let raw_input = egui_data
        .egui_winit_state
        .take_egui_input(&wgpu_data.window);

    let full_output = egui_data
        .egui_winit_state
        .egui_ctx()
        .run(raw_input, |context|{
            ui(
                UIContext { 
                    egui_context: context, 
                    event_buffers: &mut prepare_phase_context.event_buffers,
                    ui_state: &mut prepare_phase_context.ui_state
                }
            ); 
        });
    
    if egui_data.egui_winit_state.egui_ctx().has_requested_repaint() {
        wgpu_data.window.request_redraw();
    } 

    return full_output;
}
