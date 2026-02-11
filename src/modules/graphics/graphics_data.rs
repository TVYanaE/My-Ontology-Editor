pub mod graphics_backend_data;
pub mod ui_data;

use self::{
    graphics_backend_data::GraphicsBackendData,
    ui_data::UIData,
};

#[derive(Default)]
pub struct GraphicsData {
    pub graphics_backend_data: GraphicsBackendData,
    pub ui_data: UIData,
}
