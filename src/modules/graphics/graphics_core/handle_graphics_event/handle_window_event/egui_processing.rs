use winit::{
    event::WindowEvent,
};
use crate::{
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
) -> bool {
    let egui_data = egui_processing_context
        .graphics_data
        .graphics_backend_data
        .egui_data
        .as_mut()
        .unwrap();
    let wgpu_data = egui_processing_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_ref()
        .unwrap();

    let egui_response = egui_data
        .egui_winit_state.on_window_event(
            &wgpu_data.window, 
            &event
        );
 
    if egui_response.repaint {
        wgpu_data.window.request_redraw();
    } 

    return egui_response.consumed;
}
