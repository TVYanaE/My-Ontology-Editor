mod default_state_handle;
mod modal_window_state_handle;

pub use self::{
    default_state_handle::DefaultStateContext,
    modal_window_state_handle::ModalWindowStateContext,
};

pub struct UIStateHandle;
