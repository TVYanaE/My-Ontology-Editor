pub mod egui_backend;
pub mod wgpu_backend;

use self::{
    egui_backend::EGUIBackend,
    wgpu_backend::WGPUBackend,
};

#[derive(Default)]
pub struct GraphicsBackend {
    pub egui_backend: EGUIBackend, 
    pub wgpu_backend: WGPUBackend,
}
