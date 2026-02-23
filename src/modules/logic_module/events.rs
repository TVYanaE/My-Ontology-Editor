use std::{
    path::PathBuf,
    fmt::Debug,
}; 
use calloop::{
    channel::Sender
};
use uuid::{
    Uuid,
};

pub type LogicCommands = Sender<LogicCommand>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskID(Uuid);

impl TaskID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationID(Uuid);

impl ConfirmationID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub enum TaskKind {
    CreateProject{
        project_name: String,
        project_path: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub enum LogicCommand {
    Task {
        task_id: TaskID,
        task_kind: TaskKind,
    },
    ConfirmationDecision {
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind, 
    },
    Shutdown, 
}

#[derive(Debug, Clone)]
pub enum DecisionKind {
    Owerrite 
}

#[derive(Debug, Clone)]
pub enum ConfirmationKind {
    Owerrite{
        project_name: String,
        project_path: PathBuf,
    },
}


#[derive(Debug, Clone)]
pub enum TaskResult {
    Ok,
    CanceledByUser,
    Error(TaskError),  
}

#[derive(Debug, Clone)]
pub enum TaskError {
    PathError(String), 
}

impl From<ConfirmationKind> for DecisionKind {
    fn from(value: ConfirmationKind) -> Self {
        match value {
            ConfirmationKind::Owerrite { .. } => {
                Self::Owerrite
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum LogicEvent {
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

pub trait EventSender {
    type Error: Debug + Send + Sync + 'static + std::error::Error;

    fn send_event(&self, logic_event: LogicEvent) -> Result<(), Self::Error>;
}
