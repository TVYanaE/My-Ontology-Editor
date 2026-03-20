
use thiserror::Error;

use super::creating_project_event_handling::CreateProjectEventError;
use super::open_project_event_handling::OpenProjectEventError;

#[derive(Debug, Error)]
pub enum AppEventHandlingError {
    #[error("Create Project Event Error: {0}")]
    CreateProjectEventError(CreateProjectEventError),

    #[error("Open Project Event Error: {0}")]
    OpenProjectEventError(OpenProjectEventError),
}
