use std::{
    path::PathBuf,
};
use crate::{
    modules::{
        logic_module::{
            events::{
                ConfirmationID, DecisionKind
            }
        },
    },
};

pub type UIAffects = Vec<UIAffect>;

#[derive(Debug)]
pub enum UIAffect {
    ExitRequested,
    CreateProjectReq {
        project_name: String,
        project_path: PathBuf,
    },
    ConfirmationDecision {
        confirmation_id: ConfirmationID,
        decision: bool,
        decision_kind: DecisionKind, 
    },
}
