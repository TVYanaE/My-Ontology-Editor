use std::{
    path::PathBuf,
}; 

#[derive(Debug)]
pub enum LogicEvent {
    CreateProject {
        project_name: String,
        project_dir: PathBuf,
    },
    ProjectCreated,
    Shutdown,
}

