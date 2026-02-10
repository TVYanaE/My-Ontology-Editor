pub mod graphics_backend_data;

use self::{
    graphics_backend_data::GraphicsBackendData,
};

#[derive(Default)]
pub struct GraphicsData {
    pub graphics_backend_data: GraphicsBackendData,
}
