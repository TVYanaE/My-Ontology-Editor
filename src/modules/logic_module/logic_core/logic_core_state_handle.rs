mod ready_handle;
mod waiting_confirmation_handle;

pub use self::{
    ready_handle::ReadyStateContext,
    waiting_confirmation_handle::WaitingConfirmationStateContext,
};

pub struct LogicCoreStateHandle;
