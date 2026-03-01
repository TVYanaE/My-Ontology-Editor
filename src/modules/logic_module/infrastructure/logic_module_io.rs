pub mod event_manager;
pub mod event_sender;
pub mod logic_command;
pub mod logic_event;

use std::{
    path::{
        PathBuf,
    },
};
use uuid::Uuid;

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
pub struct TaskID(Uuid);

impl TaskID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
#[derive(PartialEq, Eq)]
#[derive(Hash)]
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
pub enum DecisionKind {
    Owerrite 
}

#[derive(Debug, Clone)]
pub enum ConfirmationKind {
    Owerrite{
        task_id: TaskID,
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
