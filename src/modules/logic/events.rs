use std::{
    path::PathBuf,
};

#[derive(Debug)]
pub enum LogicEvent {
    CreateProject(ProjectDescriptor),
    Shutdown,
}


#[derive(Debug)]
pub struct ProjectDescriptor {
    pub project_name: String,
    pub project_dir: PathBuf,
}




