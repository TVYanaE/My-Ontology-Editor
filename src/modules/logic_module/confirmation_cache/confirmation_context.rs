use std::{
    path::PathBuf,
};

use super::{
    super::{
        events::{
            TaskID, ConfirmationKind, 
        },
    },
};

#[derive(Debug)]
pub enum ConfirmationContext {
    CreateProjectContext {
        task_id: TaskID,
        project_name: String,
        project_path: PathBuf,
    },
}

impl From<ConfirmationKind> for ConfirmationContext {
    fn from(value: ConfirmationKind) -> Self {
        match value {
            ConfirmationKind::Owerrite { 
                task_id, 
                project_name,   
                project_path 
            } => {
                Self::CreateProjectContext { 
                    task_id, 
                    project_name, 
                    project_path 
                }
            },
        }
    }
} 
