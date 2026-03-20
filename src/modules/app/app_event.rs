pub mod creating_project_event;
pub mod open_project_event;

use super::app_kernel::app_kernel_error::AppKernelError;

use self::creating_project_event::CreatingProjectEvent;
use self::open_project_event::OpenProjectEvent;

#[derive(Debug)]
pub enum AppEvent {
    ShutdownReq, 
    CreatingProjectEvent(CreatingProjectEvent), 
    OpenProjectEvent(OpenProjectEvent), 
    KernelError(AppKernelError),
}

impl From<CreatingProjectEvent> for AppEvent {
    fn from(value: CreatingProjectEvent) -> Self {
        Self::CreatingProjectEvent(value)
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
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn drain(&mut self) -> impl Iterator<Item = AppEvent> {
        self.0.drain(..)
    }
}
