


use winit::{
    window::Window,
};

use super::{
    GraphicsCoreLogic
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
        },  
    },
};

impl GraphicsCoreLogic {
    pub fn resumed_event(
        graphics_backend: &mut GraphicsBackend,
        window: Window,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        graphics_backend.wgpu_backend.init(window)?;
        let wgpu_data = graphics_backend.wgpu_backend.get_wgpu_data()?;
        graphics_backend.egui_backend.init(wgpu_data);

        Ok(None) 
    }
}
