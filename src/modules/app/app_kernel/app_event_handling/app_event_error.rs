use thiserror::Error;

use crate::modules::app::app_kernel::app_event_handling::{
    creating_project_event_handling::CreatingProjectEventError,
    open_project_event_handling::OpenProjectEventError,
    initialisation_event_handling::InitialisationEventError, 
};

#[derive(Debug, Error)]
pub enum AppEventError {
    #[error("Creating Project Event Error: {0}")]
    CreatingProjectEventError(CreatingProjectEventError),

    #[error("Open Project Event Error: {0}")]
    OpenProjectEventError(OpenProjectEventError),

    #[error("Initialisation Event Error: {0}")]
    InitialisationEventError(InitialisationEventError),
}
