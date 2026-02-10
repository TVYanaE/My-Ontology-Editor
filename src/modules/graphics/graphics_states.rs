pub mod graphics_backend_state;

use self::{
    graphics_backend_state::GraphicsBackendState,
};

#[derive(Debug, Default)]
pub struct GraphicsStates {
    pub graphics_backend_state: GraphicsBackendState
}
