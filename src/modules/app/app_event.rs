pub mod creating_project_event;

use self::creating_project_event::CreatingProjectEvent;

#[derive(Debug, Clone)]
pub enum AppEvent {
    ShutdownReq, 
    CreatingProjectEvent(CreatingProjectEvent), 
}

impl From<CreatingProjectEvent> for AppEvent {
    fn from(value: CreatingProjectEvent) -> Self {
        Self::CreatingProjectEvent(value)
    }
}

pub struct ExternalAppEvents(Vec<AppEvent>);

impl ExternalAppEvents {
    pub fn new() -> Self {
        Self(Vec::with_capacity(8))
    }
    pub fn push(&mut self, event: AppEvent) {
        self.0.push(event);
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn drain(&mut self) -> impl Iterator<Item = AppEvent> {
        self.0.drain(..)
    }
}
