use super::{
    super::{
        events::{
            ConfirmationID,
        },
    },
    logic_core_logic::{
        WorkAfterConfirmation,
    },
};

#[derive(Debug)]
pub enum LogicCoreState {
    Ready, 
    WaitConfirmation {
        confirmation_id: ConfirmationID,
        work_after_confirmation: WorkAfterConfirmation,
    },
    Shutdown,
    Processing,
}

impl Default for LogicCoreState {
    fn default() -> Self {
        Self::Ready
    }
}
