pub mod app_event_handling_error;
mod creating_project_event_handling;
mod open_project_event_handling;

use std::sync::Arc;

use tracing::instrument;
use eframe::egui::Context as EGUIContext;

use super::AppKernel;

use super::super::app_dirs::AppDirs;
use super::app_kernel_error::AppKernelError;
use super::super::app_event::AppEvent;
use super::super::app_state::AppState;
use super::super::app_task::app_task_manager::AppTaskManager;
use super::super::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use super::super::gui::GUI;
use super::super::project::project_cache::ProjectCache;

use self::creating_project_event_handling::creating_project_event_handling;
use self::open_project_event_handling::open_project_event_handling;

impl AppKernel {
    #[instrument(
        skip(
            app_task_manager, self, gui, 
            confirmation_context_manager,
            app_dirs, project_cache,
        ), 
        err
    )]
    pub fn app_event_handling(
        &self,
        current_state: &AppState,
        event: AppEvent,
        app_task_manager: &mut AppTaskManager,
        egui_context: EGUIContext,
        gui: &mut GUI,
        confirmation_context_manager: &mut ConfirmationContextManager,
        app_dirs: Arc<AppDirs>,
        project_cache: &mut ProjectCache,
    ) -> Result<Option<AppState>, AppKernelError> {
        match current_state {
            AppState::Ready => {
                match event {
                    AppEvent::ShutdownReq => {
                        Ok(Some(AppState::Shutdown))
                    },
                    AppEvent::CreatingProjectEvent(event) => {
                        creating_project_event_handling(
                            event, 
                            app_task_manager, 
                            egui_context, 
                            gui,
                            confirmation_context_manager,
                            app_dirs,
                            project_cache,
                        ) 
                    }, 
                    AppEvent::OpenProjectEvent(event) => {
                        open_project_event_handling(
                            event,
                            app_task_manager,
                            egui_context,
                            gui,
                            project_cache,
                            app_dirs,
                        ) 
                    },
                    AppEvent::KernelError(error) => {
                        Err(error)
                    }
                }
            },
            AppState::Shutdown => {
                Ok(None)
            }
        }
    }
}
