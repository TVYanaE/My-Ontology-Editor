use std::{
    path::PathBuf,
};
use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        logic_module::{
            events::{
                ConfirmationID, DecisionKind,
                ConfirmationKind,
            },
        },
    },
};

#[derive(Debug, Clone)]
pub enum ChosedModalWindow {
    CreateNewProject{
        project_name: Option<String>,
        project_path: Option<String>,
    },
    FileDialog,
    Notification {
        text: String,
    },
    WaitingWindow {
        text: String
    },
    ConfirmationWindow {
        confirmation_id: ConfirmationID,
        confirmation_kind: ConfirmationKind,
        text: String,
    },
}


#[derive(Debug)]
pub enum UIInputEvent<'e> {
    ShowModalWindow(ChosedModalWindow), 
    PrepareUI(&'e EGUIContext),
    ShowMainUI,
}

pub type UIEvents = Vec<UIEvent>;

#[derive(Debug)]
pub enum UIEvent{
    QuitApp, 
    ModalWindowClose,
    ShowMainUI,
    ShowModalWindow(ChosedModalWindow),
    PathPicked(String),
    CreateProjectReq{
        project_name: String,
        project_path: PathBuf,
    },
    ConfirmationDecision{
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind,
    },
}
