use std::{
    path::{Path, PathBuf},
};
use tree_fs::{
    TreeBuilder,
};
use super::{
    super::{
        project::ProjectID
    },
    project_manager_error::ProjectManagerError,
    project_template::{
        ProjectTemplate,
        project_layouts::{
            project_main_files_layout::{
                ProjectMetaFileData,
            },
        },
    }
};

pub struct ProjectManagerLogic;


impl ProjectManagerLogic {
    pub fn create_new_project(
        project_name: &str,
        project_path: &impl AsRef<Path>,
        projects_dir_cache_path: &impl AsRef<Path>,
        project_template: &ProjectTemplate,
    ) -> Result<(), ProjectManagerError> {
        // Generate unique ID for Project
        let project_id = ProjectID::new();
        let project_id_str = project_id.to_string();

        let mut project_dir_cache_path = PathBuf::new();
        project_dir_cache_path.push(projects_dir_cache_path);
        project_dir_cache_path.push(project_id_str);

        // Checing and replace if project dir already exists
        // Probably make Logging collision
        if project_dir_cache_path.exists() {
            if project_dir_cache_path.is_dir() {
                std::fs::remove_dir_all(&project_dir_cache_path)?;
            }
            else {
                std::fs::remove_file(&project_dir_cache_path)?;
            }
        }

        // Creating Project Directory in project cache Directory
        std::fs::create_dir(&project_dir_cache_path)?; 

        // Creating data for project meta file 
        let project_meta_file_data = toml::to_string(
            &ProjectMetaFileData {
                project_id: project_id.clone(),
                project_name: project_name.to_string(),
            }
        )?;  

        // Creating Project Sub Directory and Main Files(without DB file) 
        let _ = TreeBuilder::default()
            .drop(false)
            .root_folder(&project_dir_cache_path)
            .add_directory(
                &project_template
                    .project_layouts
                    .project_dirs_layout
                    .semantic_nodes_catalog
                    .path
            )
            .add_file(
                &project_template
                    .project_layouts
                    .project_main_files_layout
                    .project_meta_file
                    .path, 
                &project_meta_file_data
            )
            .create()?;

        // Creating DB File

        Ok(())
    } 
}
