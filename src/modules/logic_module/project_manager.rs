mod project_manager_error; 
mod project_manager_logic;
mod project_template;

use std::{
    path::{
        Path,
    },
    sync::{
        Arc,
    },
};
use crate::{
    modules::{
        app_dirs::{
            ApplicationDirectories,
        }, 
    },
};
use super::{
    db_core::{
        DBCore,
    },
    project::{
        Project,
    },
};
use self::{
    project_manager_logic::ProjectManagerLogic,
    project_template::ProjectTemplate,
};
pub use self::{
    project_manager_error::ProjectManagerError,
};

pub struct ProjectManager {
    project_template: ProjectTemplate,
    app_dirs: Arc<ApplicationDirectories> 
}

impl ProjectManager {
    pub fn new(
        app_dirs: Arc<ApplicationDirectories>,
    ) -> Self {
        let project_template = ProjectTemplate::default();

        Self { 
            project_template: project_template,
            app_dirs: app_dirs,
        }
    }

    pub fn create_new_project(
        &self,
        project_name: &str,
        project_path: &impl AsRef<Path>,
        db_core: &mut DBCore,
    ) -> Result<Project, ProjectManagerError> {
        let project = ProjectManagerLogic::create_new_project(
            project_name, 
            project_path, 
            &self.app_dirs.as_ref().cache_directory.projects_dir_path,
            &self.project_template,
            db_core,
        )?;

        Ok(project)
    }

    pub fn open_project(
        &self,
        project_path: &impl AsRef<Path>
    ) {
        
    }
}

