use thiserror::{
    Error
};
use crate::{
    modules::{
        logic_module::{
            events::{
                EventSender,
                LogicEvent
            }
        },
        graphics_module::{
            events::{
                ExternalEvent, CustomEvents, 
                CustomEvent
            },
        },
    },
};

#[derive(Debug, Error,)]
pub enum LogicAdapterError {
    #[error("Event Loop Closed: {0}")]
    EventLoopClosed(#[from] winit::event_loop::EventLoopClosed<CustomEvent>)
}

pub struct LogicAdapter {
    custom_events: CustomEvents
}

impl LogicAdapter {
    pub fn new(custom_events: CustomEvents) -> Self {
        Self { custom_events }
    }
}

impl EventSender for LogicAdapter {
    type Error = LogicAdapterError;

    fn send_event(&self, logic_event: LogicEvent) -> Result<(), Self::Error> {
        match logic_event {
            LogicEvent::TaskRespone { 
                task_id, 
                task_result
            } => {
                self.custom_events.send_event(ExternalEvent::TaskRespone { 
                    task_id: task_id, 
                    task_result: task_result
                }.into())?;
            }, 
            LogicEvent::ConfirmationRequested { 
                confirmation_id, 
                confirmation_kind 
            } => {
                self.custom_events.send_event(ExternalEvent::ConfirmationRequested { confirmation_id, confirmation_kind }.into())?
            },
            LogicEvent::Shutdown => {
                self.custom_events.send_event(ExternalEvent::Shutdown.into())?;
            },
        } 
        Ok(())
    }
}
