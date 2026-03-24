mod check_project_info;
mod load_project_to_ram;
mod open_project_event_error;
mod unpack_project_file;

use std::sync::{Arc, RwLock};

use eframe::egui::Context as EGUIContext;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::open_project_event::OpenProjectEvent;

use crate::modules::app::app_state::AppState;

use crate::modules::app::app_task::AppAsyncTask;
use crate::modules::app::app_task::AppBlockingTask;
use crate::modules::app::app_task::app_task_manager::AppTaskManager;

use crate::modules::app::gui::GUI;
use crate::modules::app::gui::gui_command::GUICommand;

use crate::modules::app::project::projects_cache::ProjectsCache;
use crate::modules::app::project::project_manager::ProjectManager;
use crate::modules::app::project::project_view_manager::ProjectViewManager;
use crate::modules::app::project::project_view::ProjectView;

use crate::modules::app::app_dirs::AppDirs;

pub use self::open_project_event_error::OpenProjectEventError;

use self::check_project_info::{
    check_project_info,
    check_project_info_callback,
};
use self::load_project_to_ram::{
    load_project_to_ram,
    load_project_to_ram_callback,
};
use self::unpack_project_file::{
    unpack_project_file,
    unpack_project_file_callback,
};

pub fn open_project_event_handling(
    event: OpenProjectEvent, 
    ctx: OpenProjectEventHandlingContext
) -> Result<Option<AppState>, OpenProjectEventError> {
    match event {
        OpenProjectEvent::CheckProjectInfo { 
            project_file_path 
        } => {
            let app_task = AppAsyncTask {
                task: async {
                    check_project_info(project_file_path).await
                },
                callback: |result| {
                    check_project_info_callback(result)
                }
            }; 

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);
            
            ctx.gui.on_command(GUICommand::ShowLoading);

            Ok(None)
        },

        OpenProjectEvent::WrongFormat { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, has wrong format", project_file_path);
            ctx.gui.on_command(GUICommand::ShowNotification(notification));

            Ok(None)
        },

        OpenProjectEvent::SelectedFileIsntFile { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, isn't file", project_file_path);
            ctx.gui.on_command(GUICommand::ShowNotification(notification)); 
            
            Ok(None)
        },

        OpenProjectEvent::ProjectFileDoesntExists { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, doesn't exists", project_file_path);
            ctx.gui.on_command(GUICommand::ShowNotification(notification));

            Ok(None)
        },

        OpenProjectEvent::CheckProjectCache { 
            project_file_path,
            project_id,
        } => {
            let projects_cache = ctx
                .projects_cache
                .read()
                .expect("Lock Poisened. Check Project Cache.");

            match projects_cache.get_by_id(&project_id) {
                Some(project_dir_cache) => {
                    drop(projects_cache);
                    // TODO!: Logic for situation if cache has been deleted in 
                    // during of program working
                    ctx.app_task_manager.schedule_app_event(
                        AppEvent::OpenProjectEvent(
                            OpenProjectEvent::LoadProjectToRAM { 
                                project_id, 
                                project_dir_cache,
                            }
                        ),
                        ctx.egui_context,
                    ); 
                                
                    Ok(None)
                },
                None => {
                    drop(projects_cache);

                    let app_task = AppBlockingTask { 
                        task: || {
                            unpack_project_file(
                                project_file_path,
                                project_id,
                                ctx.app_dirs,
                            )
                        }, 
                        callback: |result| {
                            unpack_project_file_callback(result)
                        } 
                    };

                    ctx.app_task_manager.schedule_blocking(app_task, ctx.egui_context); 

                    Ok(None)
                },
            } 
            
        }, 

        OpenProjectEvent::PushProjectToCache { 
            project_id, 
            project_dir_cache, 
        } => {
            let mut projects_cache = ctx.projects_cache.write().unwrap();
            projects_cache.add_project(&project_id, &project_dir_cache);
            
            drop(projects_cache);

            ctx.app_task_manager.schedule_app_event(
                AppEvent::OpenProjectEvent(
                    OpenProjectEvent::LoadProjectToRAM { 
                        project_id, 
                        project_dir_cache,
                    }
                ),
                ctx.egui_context,
            ); 

            Ok(None)
        }, 

        OpenProjectEvent::LoadProjectToRAM { 
            project_id, 
            project_dir_cache,
        } => {
            let app_task = AppAsyncTask { 
                task: async {
                    load_project_to_ram(project_id, project_dir_cache).await
                }, 
                callback: |result| {
                    load_project_to_ram_callback(result)
                }, 
            };

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);

            Ok(None)
        },

        OpenProjectEvent::ProjectLoadedToRAM { 
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

pub struct OpenProjectEventHandlingContext<'c> {
    pub app_task_manager: &'c mut AppTaskManager,
    pub egui_context: EGUIContext,
    pub gui: &'c mut GUI,
    pub projects_cache: Arc<RwLock<ProjectsCache>>,
    pub app_dirs: Arc<AppDirs>,
    pub project_manager: &'c mut ProjectManager,
    pub project_view_manager: &'c mut ProjectViewManager,
}
