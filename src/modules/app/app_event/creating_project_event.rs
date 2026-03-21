use std::path::PathBuf;

use crate::modules::app::project::project_id::ProjectID;

#[derive(Debug, Clone)]
pub enum CreatingProjectEvent {
    CheckProjectInfo {
        project_name: String,
        project_path: String,
    },
    CreateProject {
        project_name: String,
        project_path: String,
    }, 
    #[allow(dead_code)]
    ProjectDirIsntExsist {
        project_name: String,
        project_path: String,   
    },
    #[allow(dead_code)]
    ProjectDirPathIsntDir {
        project_name: String,
        project_path: String,
    },
    ProjectFileAlreadyExist {
        project_name: String,
        project_path: String,
    },
    ProjectFileCreated {
        project_id: ProjectID,
        project_name: String,
        project_dir_cache: PathBuf,
    },
}
