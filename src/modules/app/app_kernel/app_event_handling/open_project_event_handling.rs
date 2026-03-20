mod check_project_info;
mod unpack_project_file;

use std::sync::Arc;

use thiserror::Error;

use eframe::egui::Context as EGUIContext;

use super::super::app_kernel_error::AppKernelError;

use super::super::super::app_event::AppEvent;

use super::super::super::app_state::AppState;
use super::super::super::app_event::open_project_event::OpenProjectEvent;

use super::super::super::app_task::AppAsyncTask;
use super::super::super::app_task::AppBlockingTask;
use super::super::super::app_task::app_task_manager::AppTaskManager;

use super::super::super::gui::GUI;
use super::super::super::gui::gui_command::GUICommand;

use super::super::super::project::project_cache::ProjectCache;

use super::super::super::app_dirs::AppDirs;

use self::check_project_info::check_project_info;
use self::check_project_info::check_project_info_callback;

use self::unpack_project_file::unpack_project_file;
use self::unpack_project_file::unpack_project_file_callback;

#[derive(Debug, Error)]
pub enum OpenProjectEventError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),
 
    #[error("Bytemuck Pod Cast Error: {0}")]
    BytemuckPodCastError(bytemuck::PodCastError),     
}

pub fn open_project_event_handling(
    event: OpenProjectEvent,
    app_task_manager: &mut AppTaskManager,
    egui_context: EGUIContext,
    gui: &mut GUI,
    project_cache: &mut ProjectCache,
    app_dirs: Arc<AppDirs>,
) -> Result<Option<AppState>, AppKernelError> {
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

            app_task_manager.schedule_async(app_task, egui_context);
 
            Ok(None)
        },

        OpenProjectEvent::WrongFormat { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, has wrong format", project_file_path);
            gui.on_command(GUICommand::ShowNotification(notification));

            Ok(None)
        },

        OpenProjectEvent::SelectedFileIsntFile { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, isn't file", project_file_path);
            gui.on_command(GUICommand::ShowNotification(notification)); 
            
            Ok(None)
        },

        OpenProjectEvent::ProjectFileDoesntExists { 
            project_file_path 
        } => {
            let notification = format!("Choosed File: {}, doesn't exists", project_file_path);
            gui.on_command(GUICommand::ShowNotification(notification));

            Ok(None)
        },

        OpenProjectEvent::CheckProjectCache { 
            project_file_path,
            project_id,
        } => {
            match project_cache.get_by_id(&project_id) {
                Some(project_dir_cache) => {
                    app_task_manager.schedule_app_event(
                        AppEvent::OpenProjectEvent(
                            OpenProjectEvent::LoadProjectToRAM { 
                                project_id: project_id, 
                                project_dir_cache: project_dir_cache,
                            }
                        ),
                        egui_context,
                    ); 
                                
                    Ok(None)
                },
                None => {
                    let app_task = AppBlockingTask { 
                        task: || {
                            unpack_project_file(
                                project_file_path,
                                project_id,
                                app_dirs,
                            )
                        }, 
                        callback: |result| {
                            unpack_project_file_callback(result)
                        } 
                    };

                    app_task_manager.schedule_blocking(app_task, egui_context); 

                    Ok(None)
                },
            } 

        }, 

        OpenProjectEvent::PushProjectToCache { 
            project_id, 
            project_dir_cache, 
        } => {
            project_cache.add_project(&project_id, &project_dir_cache);
            
            app_task_manager.schedule_app_event(
                AppEvent::OpenProjectEvent(
                    OpenProjectEvent::LoadProjectToRAM { 
                        project_id: project_id, 
                        project_dir_cache: project_dir_cache,
                    }
                ),
                egui_context,
            ); 

            Ok(None)
        }, 

        OpenProjectEvent::LoadProjectToRAM { 
            project_id, 
            project_dir_cache,
        } => {
            Ok(None)
        },
    }
}

