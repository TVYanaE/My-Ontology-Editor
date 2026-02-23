pub mod project_dirs_layout;
pub mod project_main_files_layout;

use self::{
    project_dirs_layout::ProjectDirsLayout,
    project_main_files_layout::ProjectMainFilesLayout,
};

#[derive(Debug, Default)]
pub struct ProjectLayouts {
    pub project_dirs_layout: ProjectDirsLayout,
    pub project_main_files_layout: ProjectMainFilesLayout,
}


