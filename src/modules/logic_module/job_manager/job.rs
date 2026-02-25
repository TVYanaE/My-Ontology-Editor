use std::{
    path::{
        PathBuf
    }
};
use uuid::{
    Uuid,
};
use super::{
    super::{
        events::{
            TaskID, ConfirmationID,
        },
        confirmation_cache::{
            ConfirmationContext,
        },
    },
};

#[derive(Debug)]
pub struct JobID(Uuid);

impl JobID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub struct Job {
    pub id: JobID,
    pub kind: JobKind
} 

#[derive(Debug)]
pub enum JobKind {
    Shutdown,
    CheckCreatingProjectPath {
        task_id: TaskID,
        project_name: String,
        project_path: PathBuf
    }, 
    CreateProject {
        task_id: TaskID,
        project_name: String,
        project_path: PathBuf
    },
    ConfirmationDecline {
        confirmation_context: ConfirmationContext,
    }
}
