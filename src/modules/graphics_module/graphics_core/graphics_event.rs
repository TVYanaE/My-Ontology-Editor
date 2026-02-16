use std::{
    path::PathBuf,
};
use winit::{
    window::Window,
    event::WindowEvent,
};
use winit::{
    event_loop::EventLoopProxy,
};

#[derive(Debug)]
pub enum GraphicsEvent {
    WindowEvent(WindowEvent),
    CustomEvent(CustomEvent),
}

#[derive(Debug)]
pub enum CustomEvent {
    InternalEvent(InternalEvent), 
    ExternalEvent(ExternalEvent),
}

#[derive(Debug)]
pub enum InternalEvent {
    AppShutdownReq,
    ResumedEvent(Window), 
    CreateProjectReq(CreateProjectDescriptor),
}

#[derive(Debug)]
pub struct CreateProjectDescriptor {
    pub project_name: String,
    pub project_dir: PathBuf,
}

#[derive(Debug)]
pub enum ExternalEvent { 
    AppShutdownReq,
    TaskDone,
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
impl From<ExternalEvent> for CustomEvent {
    fn from(value: ExternalEvent) -> Self {
        Self::ExternalEvent(value)
    }
}
impl From<InternalEvent> for CustomEvent {
    fn from(value: InternalEvent) -> Self {
        Self::InternalEvent(value)
    }
}


