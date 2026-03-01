mod event_manager_error;

use super::{
    event_sender::EventSender, 
    logic_event::LogicEvent, 
};
pub use self::{
    event_manager_error::EventManagerError,
};

pub struct EventManager<S: EventSender> {
    event_sender: S
}

impl<S: EventSender> EventManager<S> {
    pub fn new(event_sender: S) -> Self {
        Self { 
            event_sender 
        }
    } 

    pub fn send_event(
        &self,
        event: LogicEvent,
    ) -> Result<(), EventManagerError<S>> {
        match event {
            LogicEvent::Shutdown => {
                self.event_sender.send_event(LogicEvent::Shutdown).map_err(|error|{
                    EventManagerError::EventSenderError(error)
                })?;
                Ok(())
            },
            LogicEvent::TaskRespone { 
                task_id, 
                task_result 
            } => {
                self.event_sender.send_event(
                    LogicEvent::TaskRespone { 
                        task_id: task_id, 
                        task_result: task_result, 
                    }
                ).map_err(|error|{
                    EventManagerError::EventSenderError(error)
                })?;
                Ok(())
            },
            LogicEvent::ConfirmationRequested { 
                confirmation_id, 
                confirmation_kind 
            } => { 
                self.event_sender.send_event(
                    LogicEvent::ConfirmationRequested { 
                        confirmation_id, 
                        confirmation_kind, 
                    } 
                ).map_err(|error|{
                    EventManagerError::EventSenderError(error)
                })?;

                Ok(())
            },
        }
    }
}
