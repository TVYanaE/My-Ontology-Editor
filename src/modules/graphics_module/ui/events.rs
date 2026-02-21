use std::{
    path::PathBuf,
};
use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        shared::task_id::TaskID,
    },
};

#[derive(Debug)]
pub enum UIInputEvent<'e> {
    Waiting,
    StopWaiting,
    ShowConfirmationWindow{
        task_id: TaskID,
        text: String,
    },
    PrepareUI(&'e EGUIContext),
}

pub type UIEvents = Vec<UIEvent>;

#[derive(Debug)]
pub enum UIEvent{
    QuitApp, 
    OpenCreateNewProjectWindow,
    CloseCreateNewProjectWindow,
    OpenFileDialogReq,
    FileDialogClosed,
    ShowNotification(String),
    DirPicked(String),
    CreateProjectReq{
        project_name: String,
        project_dir: PathBuf,
    },
    Confirmation{
        task_id: TaskID,
        confirm: bool,
    },
    NotificationClosed,
}
