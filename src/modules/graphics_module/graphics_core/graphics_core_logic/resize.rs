use winit::{
    dpi::PhysicalSize,
};
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
        },  
    },
};

impl GraphicsCoreLogic {
    pub fn resize(
        physical_size: PhysicalSize<u32>, 
        graphics_backend: &mut GraphicsBackend,  
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
         
        graphics_backend.wgpu_backend.resize(physical_size)?; 
        Ok(None)
    } 
}
