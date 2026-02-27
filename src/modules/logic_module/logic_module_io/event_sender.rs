use std::{
    fmt::Debug,
};
use super::{
    logic_event::LogicEvent,
};

pub trait EventSender {
    type Error: Debug + Send + Sync + 'static + std::error::Error;

    fn send_event(&self, logic_event: LogicEvent) -> Result<(), Self::Error>;
}
