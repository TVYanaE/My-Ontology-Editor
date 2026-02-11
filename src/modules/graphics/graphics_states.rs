pub mod graphics_backend_state;
pub mod ui_state;

use self::{
    graphics_backend_state::GraphicsBackendState,
    ui_state::UIState,
};

#[derive(Debug, Default)]
pub struct GraphicsStates {
    pub graphics_backend_state: GraphicsBackendState,
    pub ui_state: UIState,
}
