mod draw_phase;

use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            graphics_backend::{
                GraphicsBackend,
            },
            ui::{
                UI, 
            },
            events::{
                CustomEvents
            },
        },  
    },
};

struct Draw;

impl GraphicsCoreLogic {
    pub fn redraw(
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
        custom_events: &CustomEvents,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        let full_output = graphics_backend.egui_backend.prepare_ui(
            wgpu_data, 
            ui, 
            custom_events
        )?;
        let egui_data = graphics_backend.egui_backend.get_mut_egui_data()?;
        Draw::draw_phase(full_output, wgpu_data, egui_data)?;

        Ok(None)
    } 
}
