mod init_projects_cache;
mod initialisation_event_error;

use std::sync::{Arc, RwLock};

use eframe::egui::Context as EGUIContext;

use crate::modules::app::app_state::AppState;

use crate::modules::app::app_event::initialisation_event::InitialisationEvent;

use crate::modules::app::project::projects_cache::ProjectsCache;

use crate::modules::app::app_task::AppBlockingTask;
use crate::modules::app::app_task::app_task_manager::AppTaskManager;

pub use initialisation_event_error::InitialisationEventError;

use self::init_projects_cache::{
    init_projects_cache,
    init_projects_cache_callbalk,
};

pub struct InitialisationEventHandlingContext<'c> {
    pub projects_cache: Arc<RwLock<ProjectsCache>>,
    pub app_task_manager: &'c mut AppTaskManager,
    pub egui_context: EGUIContext,
}

pub fn initialisation_event_handling(
    event: InitialisationEvent,
    ctx: InitialisationEventHandlingContext
) -> Result<Option<AppState>, InitialisationEventError> {
    match event {
        InitialisationEvent::InitProjectsCache => {
            let projects_cache = ctx.projects_cache.clone();
            let app_task = AppBlockingTask { 
                task: move || {
                    let mut projects_cache_lock = projects_cache
                        .write()
                        .expect("Lock Poisend. InitProjectsCache");

                    init_projects_cache(&mut projects_cache_lock)
                }, 
                callback: |result| {
                    init_projects_cache_callbalk(result) 
                } 
            };

            ctx.app_task_manager.schedule_blocking(app_task, ctx.egui_context);

            Ok(Some(AppState::Initialisation))
        },
        InitialisationEvent::InitProjectsCacheDone => {
            Ok(Some(AppState::Ready))
        },
    } 
}
