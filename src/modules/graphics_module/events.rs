use std::{
    path::PathBuf,
};
use winit::{
    window::Window,
    event::WindowEvent,
    event_loop::EventLoopProxy,
};
use crate::{
    modules::{
        shared::{
            task_id::TaskID,
        },
    },
};

pub type CustomEvents = EventLoopProxy<CustomEvent>;

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
    CreateProjectReq{
        project_name: String,
        project_dir: PathBuf,
    },
    ConfirmationObtain {
        task_id: TaskID,
        confirm: bool,
    }
}

#[derive(Debug)]
pub enum ExternalEvent { 
    AppShutdownReq,
    TaskDone(TaskID),
    ConfirmRequeired {
        task_id: TaskID,
        text: String,
    },
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

