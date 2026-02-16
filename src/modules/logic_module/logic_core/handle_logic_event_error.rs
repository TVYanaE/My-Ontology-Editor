use super::{
    LogicEvents, LogicEvent,
    logic_core_logic::LogicEventError 
}; 

pub fn handle_logic_event_error(
    error: LogicEventError,
    logic_events: &LogicEvents,
)  {
    match error {
        LogicEventError::EventLoopClosed(_) => {
            logic_events.send(LogicEvent::Shutdown).expect("Logic Event Loop Critical Erro");
        },
        LogicEventError::MPSCChannelError(_) => {
            logic_events.send(LogicEvent::Shutdown).expect("Logic Event Loop Critical Erro");
        }
    }
}
