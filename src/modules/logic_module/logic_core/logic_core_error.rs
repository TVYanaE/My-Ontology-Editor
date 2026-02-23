use thiserror::{
    Error
};
use super::{
    super::{
        events::EventSender,
        project_manager::ProjectManagerError,
    },
};

#[derive(Debug, Error)]
pub enum LogicCoreError<S: EventSender>{ 
    #[error("Event Sender Error: {0}")]
    EventSenderError(#[source] S::Error),

    #[error("Std IO Error: {0}")]
    STDIOError(#[from] std::io::Error), 

    #[error("Project Manager Error: {0}")]
    ProjectManagerError(#[from] ProjectManagerError),
}
