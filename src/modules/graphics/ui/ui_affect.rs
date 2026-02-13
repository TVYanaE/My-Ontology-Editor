use std::{
    path::PathBuf
};

#[derive(Debug)]
pub enum UIAffect{
    QuitButtonPressed, 
    CreateNewProjectButtonPressed,
    CloseCreateNewProjectWindowButtonPressed,
    CreateProjectReq(CreateProjectRequest),
}

#[derive(Debug)]
pub struct CreateProjectRequest {
    pub project_name: String, 
    pub project_dir: PathBuf, 
} 
