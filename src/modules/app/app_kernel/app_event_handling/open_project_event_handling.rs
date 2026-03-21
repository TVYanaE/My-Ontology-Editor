mod check_project_info;
mod load_project_to_ram;
mod unpack_project_file;

use std::sync::Arc;

use thiserror::Error;

use eframe::egui::Context as EGUIContext;

use crate::modules::app::app_kernel::app_kernel_error::AppKernelError;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::open_project_event::OpenProjectEvent;

use crate::modules::app::app_state::AppState;

use crate::modules::app::app_task::AppAsyncTask;
use crate::modules::app::app_task::AppBlockingTask;
use crate::modules::app::app_task::app_task_manager::AppTaskManager;

use crate::modules::app::gui::GUI;
use crate::modules::app::gui::gui_command::GUICommand;

use crate::modules::app::project::project_cache::ProjectCache;

use crate::modules::app::app_dirs::AppDirs;

use self::check_project_info::check_project_info;
use self::check_project_info::check_project_info_callback;

use self::load_project_to_ram::load_project_to_ram;
use self::load_project_to_ram::load_project_to_ram_callback;

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
    ctx: OpenProjectEventHandlingContext
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

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);
 
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
            match ctx.project_cache.get_by_id(&project_id) {
                Some(project_dir_cache) => {
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
            ctx.project_cache.add_project(&project_id, &project_dir_cache);
            
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
                    load_project_to_ram(project_dir_cache).await
                }, 
                callback: |result| {
                    load_project_to_ram_callback(result)
                }, 
            };

            ctx.app_task_manager.schedule_async(app_task, ctx.egui_context);

            Ok(None)
        },
    }
}

pub struct OpenProjectEventHandlingContext<'c> {
    pub app_task_manager: &'c mut AppTaskManager,
    pub egui_context: EGUIContext,
    pub gui: &'c mut GUI,
    pub project_cache: &'c mut ProjectCache,
    pub app_dirs: Arc<AppDirs>,
}
