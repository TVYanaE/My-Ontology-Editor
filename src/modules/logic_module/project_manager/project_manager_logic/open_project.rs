use std::{
    path::{
        Path
    },
    fs::{
        File,
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
    ) -> Result<Project, ProjectManagerError> {

         

        todo!()
    } 
}
