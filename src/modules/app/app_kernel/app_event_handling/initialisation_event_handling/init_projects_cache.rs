
use thiserror::Error;

use crate::modules::app::project::projects_cache::ProjectsCache;
use crate::modules::app::project::projects_cache::projects_cache_error::ProjectsCacheError;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::initialisation_event::InitialisationEvent;

#[derive(Debug, Error)]
pub enum InitProjectsCacheError {
    #[error("Projects Cache Error: {0}")]
    ProjectsCacheError(#[from] ProjectsCacheError),
}

pub fn init_projects_cache(
    projects_cache: &mut ProjectsCache,
) -> Result<(), InitProjectsCacheError> {
    projects_cache.scan_cache_dir()?;

    Ok(())
}

pub fn init_projects_cache_callbalk(
    result: Result<(), InitProjectsCacheError>
) -> Option<AppEvent> {
    match result {
        Ok(_) => {
            Some(
                AppEvent::InitialisationEvent(
                    InitialisationEvent::InitProjectsCacheDone
                )
            )
        },
        Err(error) => {
            None
        },
    }
}
