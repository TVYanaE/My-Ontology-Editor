pub mod app_event_handling_error;
mod creating_project_event_handling;
mod open_project_event_handling;

use std::sync::Arc;

use tracing::instrument;
use eframe::egui::Context as EGUIContext;

use crate::modules::app::AppKernel;

use crate::modules::app::app_dirs::AppDirs;
use crate::modules::app::app_kernel::app_kernel_error::AppKernelError;
use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_state::AppState;
use crate::modules::app::app_task::app_task_manager::AppTaskManager;
use crate::modules::app::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use crate::modules::app::gui::GUI;
use crate::modules::app::project::project_cache::ProjectCache;

use self::creating_project_event_handling::{
    creating_project_event_handling, CreatingProjectEventHandlingContext
};
use self::open_project_event_handling::{
    open_project_event_handling, OpenProjectEventHandlingContext
};


impl AppKernel {
    #[instrument(skip(ctx), err)]
    pub fn app_event_handling(
        event: AppEvent,
        ctx: AppEventHandlingConxtex,
    ) -> Result<Option<AppState>, AppKernelError> {
        match ctx.current_state {
            AppState::Ready => {
                match event {
                    AppEvent::ShutdownReq => {
                        Ok(Some(AppState::Shutdown))
                    },
                    AppEvent::CreatingProjectEvent(event) => {
                        creating_project_event_handling(event, ctx.into()) 
                    }, 
                    AppEvent::OpenProjectEvent(event) => {
                        open_project_event_handling(event, ctx.into()) 
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

pub struct AppEventHandlingConxtex<'c> {
    pub current_state: &'c AppState,
    pub app_task_manager: &'c mut AppTaskManager,
    pub egui_context: EGUIContext,
    pub gui: &'c mut GUI,
    pub confirmation_context_manager: &'c mut ConfirmationContextManager,
    pub app_dirs: Arc<AppDirs>,
    pub project_cache: &'c mut ProjectCache,
}

impl<'a> From<AppEventHandlingConxtex<'a>> for CreatingProjectEventHandlingContext<'a> {
    fn from(value: AppEventHandlingConxtex<'a>) -> Self {
        Self { 
            app_task_manager: value.app_task_manager, 
            egui_context: value.egui_context, 
            gui: value.gui, 
            confirmation_context_manager: value.confirmation_context_manager, 
            app_dirs: value.app_dirs, 
            project_cache: value.project_cache,
        }
    } 
}

impl<'a> From<AppEventHandlingConxtex<'a>> for OpenProjectEventHandlingContext<'a> {
    fn from(value: AppEventHandlingConxtex<'a>) -> Self {
        Self { 
            app_task_manager: value.app_task_manager, 
            egui_context: value.egui_context, 
            gui: value.gui, 
            project_cache: value.project_cache, 
            app_dirs: value.app_dirs, 
        }
    }
}
