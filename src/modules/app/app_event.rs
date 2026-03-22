pub mod creating_project_event;
pub mod initialisation_event;
pub mod open_project_event;

use crate::modules::app::app_kernel::AppEventError;

use self::creating_project_event::CreatingProjectEvent;
use self::initialisation_event::InitialisationEvent;
use self::open_project_event::OpenProjectEvent;

pub enum AppEvent {
    ShutdownReq, 
    InitialisationEvent(InitialisationEvent),
    CreatingProjectEvent(CreatingProjectEvent), 
    OpenProjectEvent(OpenProjectEvent), 
    AppEventError(AppEventError),
}

impl From<CreatingProjectEvent> for AppEvent {
    fn from(value: CreatingProjectEvent) -> Self {
        Self::CreatingProjectEvent(value)
    }
}

impl From<InitialisationEvent> for AppEvent {
    fn from(value: InitialisationEvent) -> Self {
        Self::InitialisationEvent(value)
    }
}

impl From<OpenProjectEvent> for AppEvent {
    fn from(value: OpenProjectEvent) -> Self {
        Self::OpenProjectEvent(value)
    }
}

pub struct AppEvents(Vec<AppEvent>);

impl AppEvents {
    pub fn new() -> Self {
        Self(Vec::with_capacity(8))
    }
    pub fn push(&mut self, event: AppEvent) {
        self.0.push(event);
    } 
    pub fn drain(&mut self) -> impl Iterator<Item = AppEvent> {
        self.0.drain(..)
    }
}
