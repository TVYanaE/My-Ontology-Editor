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
pub struct TaskID(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfirmationID(pub Uuid);

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
pub enum ResultKind {
    Ok,
    Error(ErrorKind),  
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
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
        result: ResultKind,
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
