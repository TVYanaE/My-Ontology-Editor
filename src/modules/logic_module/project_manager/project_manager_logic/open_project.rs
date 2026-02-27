use std::{
    path::{
        Path
    },
    fs::{
        File,
    },
};

use crate::{
    modules::{
        db_module::{
            DBCommands, DBCommand
        },
    },
};
use super::{
    super::{
        super::{
            project::{
                Project,
            }, 
        },
        project_manager_error::ProjectManagerError,
    },
    ProjectManagerLogic
};

impl ProjectManagerLogic {
    pub fn open_project(
        project_path: &impl AsRef<Path>,
        db_commands: &DBCommands,
    ) -> Result<Project, ProjectManagerError> {

         

        todo!()
    } 
}
