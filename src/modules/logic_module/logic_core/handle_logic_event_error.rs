use super::{
    LogicEvents, LogicEvent,
    logic_core_logic::LogicEventError, 
    CustomEvents, ExternalEvent,
}; 

// TODO: Send signal to Graphics Module for shutdown

pub fn handle_logic_event_error(
    error: LogicEventError,
    logic_events: &LogicEvents,
    custom_events: &CustomEvents,
)  {
    match error {
        LogicEventError::EventLoopClosed(_) => {
            logic_events.send(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error");
            custom_events.send_event(ExternalEvent::AppShutdownReq.into())
                .expect("Winit Event Loop closed");
        },
        LogicEventError::MPSCChannelError(_) => {
            logic_events.send(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error");
            custom_events.send_event(ExternalEvent::AppShutdownReq.into())
                .expect("Winit Event Loop closed");
        },
        LogicEventError::STDIOError(_) => {
            logic_events.send(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error");
            custom_events.send_event(ExternalEvent::AppShutdownReq.into())
                .expect("Winit Event Loop closed");
        },
        LogicEventError::ProjectManagerError(_) => {
            logic_events.send(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error");
            custom_events.send_event(ExternalEvent::AppShutdownReq.into())
                .expect("Winit Event Loop closed");
        }, 
        LogicEventError::RwLockPoisonError => {
            logic_events.send(LogicEvent::Shutdown)
                .expect("Logic Event Loop Critical Error");
            custom_events.send_event(ExternalEvent::AppShutdownReq.into())
                .expect("Winit Event Loop closed");
        }
    }
}
