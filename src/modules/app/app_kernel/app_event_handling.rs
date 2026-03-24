mod app_event_error;
mod creating_project_event_handling;
mod initialisation_event_handling;
mod open_project_event_handling;

use std::sync::{Arc, RwLock};

use tracing::instrument;
use eframe::egui::Context as EGUIContext;

use crate::modules::app::AppKernel;

use crate::modules::app::app_dirs::AppDirs;
use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_state::AppState;
use crate::modules::app::app_task::app_task_manager::AppTaskManager;
use crate::modules::app::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use crate::modules::app::gui::GUI;

use crate::modules::app::project::projects_cache::ProjectsCache;
use crate::modules::app::project::project_manager::ProjectManager;
use crate::modules::app::project::project_view_manager::ProjectViewManager;

pub use app_event_error::AppEventError;
pub use creating_project_event_handling::CreatingProjectEventError;
pub use initialisation_event_handling::InitialisationEventError;
pub use open_project_event_handling::OpenProjectEventError;

use self::creating_project_event_handling::{
    creating_project_event_handling, CreatingProjectEventHandlingContext
};
use self::initialisation_event_handling::{
    initialisation_event_handling, InitialisationEventHandlingContext,
};
use self::open_project_event_handling::{
    open_project_event_handling, OpenProjectEventHandlingContext
};


impl AppKernel {
    #[instrument(skip(ctx, event), err)]
    pub fn app_event_handling(
        event: AppEvent,
        ctx: AppEventHandlingConxtex,
    ) -> Result<Option<AppState>, AppEventError> {
        match ctx.current_state {
            AppState::NotInit => {
                match event {
                    AppEvent::InitialisationEvent(event) => {
                        initialisation_event_handling(event, ctx.into()).map_err(|error|{
                            AppEventError::InitialisationEventError(error)
                        })
                    },
                    _ => Ok(None),
                }
            },
            AppState::Initialisation => {
                match event {
                    AppEvent::InitialisationEvent(event) => {
                        initialisation_event_handling(event, ctx.into()).map_err(|error|{
                            AppEventError::InitialisationEventError(error)
                        })
                    },
                    _ => {
                        Ok(None)
                    }
                }
            },
            AppState::Ready => {
                match event {
                    AppEvent::ShutdownReq => {
                        Ok(Some(AppState::Shutdown))
                    },
                    AppEvent::CreatingProjectEvent(event) => {
                        creating_project_event_handling(event, ctx.into()).map_err(|error|{
                            AppEventError::CreatingProjectEventError(error)
                        }) 
                    }, 
                    AppEvent::OpenProjectEvent(event) => {
                        open_project_event_handling(event, ctx.into()).map_err(|error|{
                            AppEventError::OpenProjectEventError(error)
                        })
                    },
                    AppEvent::AppEventError(error) => {
                        Err(error)
                    },
                    _ => {
                        Ok(None)
                    },
                }
            },
            AppState::Shutdown => {
                Ok(None)
            },
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
    pub projects_cache: Arc<RwLock<ProjectsCache>>,
    pub project_manager: &'c mut ProjectManager,
    pub project_view_manager: &'c mut ProjectViewManager,
}

impl<'a> From<AppEventHandlingConxtex<'a>> for CreatingProjectEventHandlingContext<'a> {
    fn from(value: AppEventHandlingConxtex<'a>) -> Self {
        Self { 
            app_task_manager: value.app_task_manager, 
            egui_context: value.egui_context, 
            gui: value.gui, 
            confirmation_context_manager: value.confirmation_context_manager, 
            app_dirs: value.app_dirs, 
            projects_cache: value.projects_cache,
            project_manager: value.project_manager,
            project_view_manager: value.project_view_manager,
        }
    } 
}

impl<'a> From<AppEventHandlingConxtex<'a>> for InitialisationEventHandlingContext<'a> {
    fn from(value: AppEventHandlingConxtex<'a>) -> Self {
        Self { 
            projects_cache: value.projects_cache,
            egui_context: value.egui_context,
            app_task_manager: value.app_task_manager,
        }
    }
}

impl<'a> From<AppEventHandlingConxtex<'a>> for OpenProjectEventHandlingContext<'a> {
    fn from(value: AppEventHandlingConxtex<'a>) -> Self {
        Self { 
            app_task_manager: value.app_task_manager, 
            egui_context: value.egui_context, 
            gui: value.gui, 
            projects_cache: value.projects_cache, 
            app_dirs: value.app_dirs, 
            project_manager: value.project_manager,
            project_view_manager: value.project_view_manager
        }
    }
}
