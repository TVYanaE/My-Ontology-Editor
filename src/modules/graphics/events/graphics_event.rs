use winit::{
    window::Window,
    event::WindowEvent,
};

#[derive(Debug)]
pub enum GraphicsEvent {
    WindowEvent(WindowEvent),
    CustomEvent(CustomEvent),
}

#[derive(Debug)]
pub enum CustomEvent {
    AppShutdownReq,
    ResumedEvent(Window),
    ITCEvent(ITCEvent)
}

#[derive(Debug)]
pub enum ITCEvent { 
    AppShutdownReq,
}

impl From<WindowEvent> for GraphicsEvent {
    fn from(value: WindowEvent) -> Self {
        Self::WindowEvent(value)
    }
}

impl From<CustomEvent> for GraphicsEvent {
    fn from(value: CustomEvent) -> Self {
        Self::CustomEvent(value)
    }
}

impl From<ITCEvent> for CustomEvent {
    fn from(value: ITCEvent) -> Self {
        Self::ITCEvent(value)
    }
}
