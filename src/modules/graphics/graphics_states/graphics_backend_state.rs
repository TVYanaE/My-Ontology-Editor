#[derive(Debug, Default)]
pub struct GraphicsBackendState {
    pub egui_state: EGUIState, 
    pub wgpu_state: WGPUState,
}

#[derive(Debug)]
pub enum EGUIState {
    Processing,
    NotInit,
    Init,
}

impl Default for EGUIState {
    fn default() -> Self {
        Self::NotInit
    }
}

#[derive(Debug)]
pub enum WGPUState {
    Processing,
    NotInit,
    Init
}

impl Default for WGPUState {
    fn default() -> Self {
        Self::NotInit
    }
}
