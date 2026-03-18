mod check_project_info; 
mod create_project_file;

use std::sync::Arc;

use eframe::egui::Context as EGUIContext;

use super::super::super::app_event::creating_project_event::CreatingProjectEvent;
use super::super::super::app_state::AppState;
use super::super::app_kernel_error::AppKernelError;
use super::super::super::app_task::{AppBlockingTask, AppAsyncTask};
use super::super::super::app_task::app_task_manager::AppTaskManager;
use super::super::super::confirmation_context::{
    ConfirmationContext, CONFIRMATION_CONTEXT_ID_GENERATOR,
};
use super::super::super::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use super::super::super::gui::GUI;
use super::super::super::gui::gui_command::{GUICommand, ConfirmationType};
use super::super::super::app_dirs::AppDirs;

use self::check_project_info::{
    check_project_info,
    check_project_info_callback,
};
use create_project_file::{
    create_project_file,
};

pub fn creating_project_event_handling(
    event: CreatingProjectEvent,
    app_task_manager: &mut AppTaskManager,
    egui_context: EGUIContext,
    gui: &mut GUI,
    confirmation_context_manager: &mut ConfirmationContextManager,
    app_dirs: Arc<AppDirs>,
) -> Result<Option<AppState>, AppKernelError> {
    match event {
        CreatingProjectEvent::CheckProjectInfo { 
            project_name, 
            project_path 
        } => {
            let app_task = AppBlockingTask {
                task: move || { 
                    let result = check_project_info(project_name, project_path);
                    Box::new(result) 
                },
                callback: move |response| {
                    check_project_info_callback(response) 
                }, 
            };
            app_task_manager.schedule_blocking(app_task, egui_context);

            Ok(None)
        }, 

        CreatingProjectEvent::CreateProject { 
            project_name, 
            project_path, 
        } => {
            let app_task = AppAsyncTask {
                task: async {
                    create_project_file(project_name, project_path, app_dirs).await
                },
                callback: |res| {
                    None
                }
            };

            app_task_manager.schedule_async(app_task, egui_context);

            Ok(None) 
        },

        CreatingProjectEvent::ProjectFileAlreadyExist {
            project_name,
            project_path,
        } => {
            let confirmation_context_id = CONFIRMATION_CONTEXT_ID_GENERATOR.next();

            confirmation_context_manager.push(
                confirmation_context_id.clone(), 
                ConfirmationContext::OverwriteProjectFileContext { 
                    project_name: project_name, 
                    project_path: project_path, 
                }
            );

            gui.on_command(
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
            gui.on_command(
                GUICommand::ShowNotification(
                    "Choosed Directory For Project File isn't Exsist".into()
                )
            ); 

            Ok(None) 
        },

        CreatingProjectEvent::ProjectDirPathIsntDir {..} => {
            gui.on_command(
                GUICommand::ShowNotification(
                    "Choosed Directory For Project File isn't Directory".into()
                )
            );

            Ok(None) 
        },
    }
}
