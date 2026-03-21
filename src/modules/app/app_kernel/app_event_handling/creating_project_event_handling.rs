mod check_project_info; 
mod create_project_file;
mod load_project_to_ram;

use std::sync::Arc;

use thiserror::Error;

use eframe::egui::Context as EGUIContext;

use crate::modules::app::app_event::creating_project_event::CreatingProjectEvent;
use crate::modules::app::app_state::AppState;

use crate::modules::app::app_kernel::app_kernel_error::AppKernelError;

use crate::modules::app::app_task::{
    AppBlockingTask, AppAsyncTask,
    app_task_manager::AppTaskManager,
};

use crate::modules::app::confirmation_context::{
    ConfirmationContext, CONFIRMATION_CONTEXT_ID_GENERATOR,
    confirmation_context_manager::ConfirmationContextManager
};

use crate::modules::app::gui::{
    GUI,
    gui_command::{GUICommand, ConfirmationType},
};

use crate::modules::app::app_dirs::AppDirs;

use crate::modules::app::project::{
    Project,
    project_cache::ProjectCache,
};

use self::check_project_info::{
    check_project_info,
    check_project_info_callback,
};
use create_project_file::{
    create_project_file,
    create_project_file_callback,
};

#[derive(Debug, Error)]
pub enum CreateProjectEventError {
    #[error("STD IO Error: {0}")]
    STDIO(#[from] std::io::Error),

    #[error("SQLX Error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Toml Crate Error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Strip Prefix Error: {0}")]
    StripPrefix(#[from] std::path::StripPrefixError),  
}

pub fn creating_project_event_handling(
    event: CreatingProjectEvent,
    ctx: CreatingProjectEventHandlingContext 
) -> Result<Option<AppState>, AppKernelError> {
    match event {
        CreatingProjectEvent::CheckProjectInfo { 
            project_name, 
            project_path 
        } => {
            let app_task = AppBlockingTask {
                task: move || { 
                    check_project_info(project_name, project_path)
                },
                callback: move |response| {
                    check_project_info_callback(response) 
                }, 
            };
            ctx.app_task_manager.schedule_blocking(app_task, ctx.egui_context);

            ctx.gui.on_command(GUICommand::ShowLoading);

            Ok(None)
        }, 

        CreatingProjectEvent::CreateProject { 
            project_name, 
            project_path, 
        } => {
            let app_task = AppAsyncTask {
                task: async { 
                    create_project_file(project_name, project_path, ctx.app_dirs).await
                },
                callback: |res| {
                    create_project_file_callback(res)
                }
            };

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);

            Ok(None) 
        },

        CreatingProjectEvent::ProjectFileAlreadyExist {
            project_name,
            project_path,
        } => {
            let confirmation_context_id = CONFIRMATION_CONTEXT_ID_GENERATOR.next();

            ctx.confirmation_context_manager.push(
                confirmation_context_id.clone(), 
                ConfirmationContext::OverwriteProjectFileContext { 
                    project_name, 
                    project_path, 
                }
            );

            ctx.gui.on_command(
                GUICommand::ShowConfirmationWindow { 
                    confirmation_type: ConfirmationType::OverwriteProjectFile(
                        confirmation_context_id
                    ), 
                    confirmation_text: "Project file already exist".to_string(), 
                }
            );

            Ok(None) 
        },

        CreatingProjectEvent::ProjectDirIsntExsist {..} => {
            ctx.gui.on_command(
                GUICommand::ShowNotification(
                    "Choosed Directory For Project File isn't Exsist".into()
                )
            ); 

            Ok(None) 
        },

        CreatingProjectEvent::ProjectDirPathIsntDir {..} => {
            ctx.gui.on_command(
                GUICommand::ShowNotification(
                    "Choosed Directory For Project File isn't Directory".into()
                )
            );

            Ok(None) 
        },

        CreatingProjectEvent::ProjectFileCreated { 
            project_id,
            project_name,
            project_dir_cache,
        } => {
            ctx.project_cache.add_project(&project_id, &project_dir_cache);

            let app_task = AppAsyncTask {
                task: async move {
                    Project::new(&project_dir_cache.clone()).await 
                },
                callback: |res| {
                    if let Ok(project) = res { 
                        println!("Test project load");
                        println!("Project name: {}", project.get_project_name());
                    }

                    None
                }
            }; 

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);
            
            // TODO!: Replace to final stage  
            ctx.gui.on_command(GUICommand::StopShowLoading);

            Ok(None)
        },
    }
}

pub struct CreatingProjectEventHandlingContext<'c> {
    pub app_task_manager: &'c mut AppTaskManager,
    pub egui_context: EGUIContext,
    pub gui: &'c mut GUI,
    pub confirmation_context_manager: &'c mut ConfirmationContextManager,
    pub app_dirs: Arc<AppDirs>,
    pub project_cache: &'c mut ProjectCache,
} 
