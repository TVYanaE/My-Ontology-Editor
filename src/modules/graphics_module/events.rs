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
        logic_module::{
            events::{
                TaskID, TaskKind,
                ConfirmationID, ConfirmationKind,
                DecisionKind,
                TaskResult,
            }
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
    ShutdownReq,
    ResumedEvent(Window), 
    CreateProjectReq{
        project_name: String,
        project_path: PathBuf,
    },
    ConfirmationDecision {
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind,
    }
}

#[derive(Debug)]
pub enum ExternalEvent { 
    TaskRespone {
        task_id: TaskID,
        task_kind: TaskKind,
        task_result: TaskResult,
    }, 
    ConfirmationRequested {
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
    }, 
    Shutdown, 
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

