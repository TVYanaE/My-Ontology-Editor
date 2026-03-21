use thiserror::Error;

use crate::modules::app::app_kernel::app_event_handling::{
    creating_project_event_handling::CreateProjectEventError,
    open_project_event_handling::OpenProjectEventError,
};

#[derive(Debug, Error)]
pub enum AppEventHandlingError {
    #[error("Create Project Event Error: {0}")]
    CreateProjectEventError(CreateProjectEventError),

    #[error("Open Project Event Error: {0}")]
    OpenProjectEventError(OpenProjectEventError),
}
