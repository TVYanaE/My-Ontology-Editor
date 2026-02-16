use std::{
    path::PathBuf,
};

pub type UIEvents = Vec<UIEvent>;

#[derive(Debug)]
pub enum UIEvent{
    QuitButtonPressed, 
    CreateNewProjectButtonPressed,
    CloseCreateNewProjectWindowButtonPressed,
    OpenFileDialogReq,
    FileDialogClosed,
    DirPicked(String),
    CreateProjectReq(CreateProjectRequest),
    Error(String),
    NotificationClosed,
}

#[derive(Debug)]
pub struct CreateProjectRequest {
    pub project_name: String, 
    pub project_dir: PathBuf, 
} 
