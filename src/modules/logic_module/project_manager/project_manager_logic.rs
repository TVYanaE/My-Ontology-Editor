use std::{
    fs::{
        self, File,
    }, 
    path::{PathBuf, Path},
};
use tar::{
    Builder,
};
use oneshot::{
    Sender, channel,
};
use tree_fs::{
    TreeBuilder,
};
use uuid::{
    Uuid,
};
use crate::{
    modules::{
        graphics_module::{
            ExternalEvent,
        },
    },
};
use super::{
    DBEvents, Project, CustomEvents, 
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

pub struct ProjectManagerLogic;

pub struct CreateUnpackedProjectContext<'c> {
    pub projects_dir_cache_path: PathBuf,
    pub project_name: String,
    pub project_dir: PathBuf,
    pub db_events: DBEvents,
    pub custom_events: &'c CustomEvents,
}

impl ProjectManagerLogic {
    pub fn create_unpacked_project(
        context: CreateUnpackedProjectContext,
    ) -> Result<Project, ProjectManagerError> {

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

        create_main_dirs_files(
            &project_cache_dir, 
            &project_dirs_layout, 
            &project_main_files_layout, 
            &project_main_files_payloads
        )?;

        let project = Project::new(
            &project_cache_dir, 
            &project_dirs_layout.semantic_nodes_catalog.path, 
            &project_main_files_layout.project_meta_file.path, 
            context.db_events,
            project_id
        )?;

        pack_project(
            &project, 
            &context.project_dir, 
            &context.project_name,
            context.custom_events
        )?;

        Ok(project) 
    }
 
}

fn create_main_dirs_files(
    root: & impl AsRef<Path>,
    project_dirs_layout: &ProjectDirsLayout,
    project_main_files_layout: &ProjectMainFilesLayout,
    project_main_files_payloads: &ProjectMainFilesPayloads,
) -> Result<(), ProjectManagerError> {
    TreeBuilder::default()
        .drop(false)
        .root_folder(root)
        .add_directory(&project_dirs_layout.semantic_nodes_catalog.path)
        .add_file(
            &project_main_files_layout.project_meta_file.path, 
            &project_main_files_payloads.meta_file.data
        )
        .create()?;

    Ok(())
}  

fn pack_project(
    project: &Project,
    project_dir: &impl AsRef<Path>,
    project_name: &str,
    custom_events: &CustomEvents,
) -> Result<(), ProjectManagerError> {
    // Create destination project file 
    
    let mut project_file_path = project_dir.as_ref().to_path_buf(); 
    project_file_path.push(project_name);
    project_file_path.set_extension("vontov");

   /*  if project_file_path.exists() {
        let (sender, recv) = channel::<bool>(); 
        custom_events.send_event(ExternalEvent::ConfirmRequeired { task_id: (), text: () } { 
            text: "Project Already Exsist. Replace?".into(), 
            response_targer: sender, 
        }.into()
        )?; 

        if !recv.recv()? {
            return Ok(());
        }
    } */

    // TODO: Create handling of existing file
    let project_file = File::create_new(&project_file_path)?;
     
    // Logic for replace existing project 

    let project_root = project.get_project_root();

    let mut archive_builder = Builder::new(project_file);

    archive_builder.append_dir_all("", project_root)?;

    archive_builder.finish()?;

    Ok(())
}
