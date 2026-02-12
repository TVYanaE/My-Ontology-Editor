use egui::{
    FullOutput
};
use super::{
    WindowEventError,
};
use crate::{
    modules::{
        graphics::{
            events::EventBuffers,
            graphics_data::{
                graphics_backend_data::{
                    wgpu_data::WGPUData,
                    egui_data::EGUIData,
                },
                ui_data::UIData,
            },
            graphics_states::ui_state::UIState,
            ui::{ui, UIContext},
        },
    },
};

pub struct PreparePhaseContext<'c> {
    pub wgpu_data: &'c WGPUData,
    pub egui_data: &'c mut EGUIData,
    pub ui_data: &'c mut UIData,
    pub event_buffers: &'c mut EventBuffers,
    pub ui_state: &'c mut UIState,
}

pub fn prepare_phase(
    mut prepare_phase_context: PreparePhaseContext,
) -> FullOutput {
    let raw_input = prepare_phase_context.egui_data
        .egui_winit_state
        .take_egui_input(&prepare_phase_context.wgpu_data.window);

    let full_output = prepare_phase_context.egui_data
        .egui_winit_state
        .egui_ctx()
        .run(raw_input, |context|{
            ui(
                UIContext { 
                    egui_context: context, 
                    event_buffers: &mut prepare_phase_context.event_buffers,
                    ui_state: &mut prepare_phase_context.ui_state,
                    ui_data: prepare_phase_context.ui_data
                }
            ); 
        }); 

    return full_output;
}
