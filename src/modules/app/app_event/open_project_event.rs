use std::path::PathBuf;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::Project;

pub enum OpenProjectEvent {
    CheckProjectInfo {
        project_file_path: String,
    },  
    ProjectFileDoesntExists {
        project_file_path: String,
    },
    SelectedFileIsntFile {
        project_file_path: String,
    },
    WrongFormat {
        project_file_path: String,
    },
    CheckProjectCache {
        project_file_path: String,
        project_id: ProjectID,
    },
    PushProjectToCache {
        project_id: ProjectID,
        project_dir_cache: PathBuf,
    }, 
    LoadProjectToRAM {
        project_id: ProjectID,
        project_dir_cache: PathBuf,
    },
    ProjectLoadedToRAM {
        project_id: ProjectID,
        project: Project,
    }
}
