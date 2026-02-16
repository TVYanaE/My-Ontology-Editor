use crate::{
    modules::{
        logic_module::{
            logic_core::{
                event_loop::{
                    handle_logic_event::{
                        LogicEventError
                    },
                }, 
                LogicCoreState,
            },
        },
    },
};

pub fn handle_logic_event_error(
    error: LogicEventError
) -> Option<LogicCoreState> {
    match error {
        LogicEventError::EventLoopClosed(_) => {
            Some(LogicCoreState::Shutdown)
        },
    }
}
