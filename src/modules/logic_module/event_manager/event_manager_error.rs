use thiserror::{
    Error
};
use super::{
    super::{
        events::EventSender
    },
};

#[derive(Debug, Error)]
pub enum EventManagerError<S: EventSender>{
    #[error("Event Sender Error: {0}")]
    EventSenderError(#[source] S::Error), 
}
