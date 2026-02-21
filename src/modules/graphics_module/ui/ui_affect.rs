use std::{
    path::PathBuf,
};
use crate::{
    modules::{
        shared::{
            task_id::TaskID,
        },
    },
};

pub type UIAffects = Vec<UIAffect>;

#[derive(Debug)]
pub enum UIAffect {
    ExitRequested,
    CreateProjectReq {
        project_name: String,
        project_dir: PathBuf,
    },
    Confirmation {
        task_id: TaskID,
        confirm: bool,
    },
}
