pub mod egui_data;
pub mod wgpu_data;

use self::{
    egui_data::EGUIData,
    wgpu_data::WGPUData,
};

#[derive(Default)]
pub struct GraphicsBackendData {
    pub egui_data: Option<EGUIData>,
    pub wgpu_data: Option<WGPUData>,
}
