use std::path::PathBuf;

use super::super::project::project_id::ProjectID;

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
    ProjectDirIsntExsist {
        project_name: String,
        project_path: String,   
    },
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
