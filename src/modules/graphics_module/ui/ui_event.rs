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
    CreateProjectReq{
        project_name: String,
        project_dir: PathBuf,
    },
    Error(String),
    NotificationClosed,
}
