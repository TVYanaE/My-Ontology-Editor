mod check_project_info; 
mod create_project_file;
mod creating_project_event_error;
mod load_project_to_ram;

use std::sync::{Arc, RwLock};


use eframe::egui::Context as EGUIContext;

use crate::modules::app::app_event::creating_project_event::CreatingProjectEvent;
use crate::modules::app::app_state::AppState;

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
    project_manager::ProjectManager,
    projects_cache::ProjectsCache,
    project_view_manager::ProjectViewManager,
    project_view::ProjectView,
};

pub use self::creating_project_event_error::CreatingProjectEventError;

use self::check_project_info::{
    check_project_info,
    check_project_info_callback,
};
use create_project_file::{
    create_project_file,
    create_project_file_callback,
};
use self::load_project_to_ram::{
    load_project_to_ram,
    load_project_to_ram_callback,
};


pub fn creating_project_event_handling(
    event: CreatingProjectEvent,
    ctx: CreatingProjectEventHandlingContext 
) -> Result<Option<AppState>, CreatingProjectEventError> {
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
            project_dir_cache,
        } => {
            let mut projects_cache = ctx
                .projects_cache
                .write()
                .expect("Lock Poisened Project File Created");
            projects_cache.add_project(&project_id, &project_dir_cache);

            drop(projects_cache);

            let app_task = AppAsyncTask {
                task: async move {
                    load_project_to_ram(project_id, project_dir_cache).await
                },
                callback: |result| {
                    load_project_to_ram_callback(result) 
                }
            }; 

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);
             
            Ok(None)
        },

        CreatingProjectEvent::ProjectLoadedToRAM { 
            project_id, 
            project,
        } => {
            let project_name = project.get_project_name().to_string();
            
            let project_view = ProjectView::new(project_id.clone(), project_name);

            ctx.project_view_manager.push(project_id.clone(), project_view);

            ctx.project_manager.push(project_id, project); 
            
            ctx.gui.on_command(GUICommand::StopShowLoading);
            ctx.gui.on_command(GUICommand::ShowMainUI);

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
    pub projects_cache: Arc<RwLock<ProjectsCache>>,
    pub project_manager: &'c mut ProjectManager,
    pub project_view_manager: &'c mut ProjectViewManager,
} 
