use tracing::{
    error,
};
use super::{
    super::{
        infrastructure::{
            logic_module_io::{
                event_sender::EventSender,
                logic_event::LogicEvent,
                event_manager::{EventManager, EventManagerError},
            },
        },
    },
    LogicCoreState,
    logic_core_error::LogicCoreError,
}; 

pub fn logic_core_error_handle<S>(
    error: LogicCoreError<S>,
    event_manager: &EventManager<S>,
) -> Option<LogicCoreState>  
where 
    S: EventSender
{
    match error {   
        LogicCoreError::STDIOError(_) => {
            graphic_module_shutdown(event_manager); 
            Some(LogicCoreState::Shutdown)
        },  
        LogicCoreError::ProjectManagerError(_) => {
            graphic_module_shutdown(event_manager); 
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::EventManagerError(_) => {
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::JobManagerError(_) => {
            graphic_module_shutdown(event_manager); 
            Some(LogicCoreState::Shutdown)
        },
    }
}


fn graphic_module_shutdown<S: EventSender>(
    event_manager: &EventManager<S>,
) {
    match event_manager.send_event(LogicEvent::Shutdown) {
        Ok(_) => {},
        Err(error) => {
            match error {
                EventManagerError::EventSenderError(error) => {
                    error!(error=?error, "Graphic Thread panic");
                },
            }
        },
    }
}
