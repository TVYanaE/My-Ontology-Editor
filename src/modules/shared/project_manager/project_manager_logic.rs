use std::{
    fs, 
    path::{PathBuf, Path},
};
use tree_fs::{
    TreeBuilder,
};
use uuid::{
    Uuid,
};

pub struct ProjectManagerLogic;

use super::{
    project_layouts::{
        project_dirs_layout::ProjectDirsLayout,
        project_main_files_layout::ProjectMainFilesLayout,
    },
    project_main_files_payloads::{
        MainFilesPayloadsDescriptor,
        ProjectMainFilesPayloads,
    },
    ProjectManagerError, 
};

pub struct CreateProjectContext {
    pub projects_dir_cache_path: PathBuf,
    pub project_name: String,
    pub project_dir: PathBuf,
}

impl ProjectManagerLogic {
    pub fn create_project(
        context: CreateProjectContext,
    ) -> Result<(), ProjectManagerError> {

        // Generate project unique ID
        let project_id = Uuid::new_v4();

        // Create sub directory for project 
        let mut project_cache_dir = context
            .projects_dir_cache_path
            .clone();

        let project_id_string = project_id.to_string();

        project_cache_dir.push(&project_id_string);

        // Check ID collisions 
        // TODO: Have to thik about regenerate ID 
        if project_cache_dir.exists() {
            fs::remove_dir_all(&project_cache_dir)?;
        }

        fs::create_dir(&project_cache_dir)?;

        // Create project layouts
        let project_dirs_layout = ProjectDirsLayout::create_defaul_dirs_layout();
        let project_main_files_layout = ProjectMainFilesLayout::create_default_main_files_layout();

        // Create project main files payloads
        let project_main_files_payloads = ProjectMainFilesPayloads::create(
            MainFilesPayloadsDescriptor { 
                project_id, 
                project_name: &context.project_name 
            }
        )?;

        create_unpacked_project(
            &project_cache_dir, 
            project_dirs_layout, 
            project_main_files_layout, 
            project_main_files_payloads
        )?;
        Ok(()) 
    }
}

fn create_unpacked_project(
    root: & impl AsRef<Path>,
    project_dirs_layout: ProjectDirsLayout,
    project_main_files_layout: ProjectMainFilesLayout,
    project_main_files_payloads: ProjectMainFilesPayloads,
) -> Result<(), ProjectManagerError> {
    TreeBuilder::default()
        .drop(false)
        .root_folder(root)
        .add_directory(project_dirs_layout.semantic_nodes_catalog.path)
        .add_file(
            project_main_files_layout.project_meta_file.path, 
            &project_main_files_payloads.meta_file.data
        )
        .create()?;
    Ok(())
}  
