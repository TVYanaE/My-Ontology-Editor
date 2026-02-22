use super::{
    super::{
        events::{EventSender, LogicEvent}, 
    },
    LogicCoreError, 
    LogicCoreState,
}; 

pub fn logic_core_error_handle<S>(
    error: LogicCoreError<S>,
    sender: &S 
) -> Option<LogicCoreState>  
where 
    S: EventSender
{
    match error { 
        LogicCoreError::EventSenderError(_) => {
            Some(LogicCoreState::Shutdown)
        }, 
        LogicCoreError::MPSCChannelDBEventError(_) => {
            sender.send_event(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error"); 
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::STDIOError(_) => {
            sender.send_event(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error"); 
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::ProjectManagerError(_) => {
            sender.send_event(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error"); 
            Some(LogicCoreState::Shutdown)
        },  
    }
}
