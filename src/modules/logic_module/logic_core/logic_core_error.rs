use thiserror::{
    Error
};
use super::{
    super::{
        events::EventSender,
        project_manager::ProjectManagerError,
        job_manager::JobManagerError,
        event_manager::EventManagerError,
    },
};

#[derive(Debug, Error)]
pub enum LogicCoreError<S: EventSender>{ 
    #[error("Std IO Error: {0}")]
    STDIOError(#[from] std::io::Error), 

    #[error("Project Manager Error: {0}")]
    ProjectManagerError(#[from] ProjectManagerError),

    #[error("Job Manager Error: {0}")]
    JobManagerError(#[from] JobManagerError),

    #[error("Event Manager Error: {0}")]
    EventManagerError(#[from] EventManagerError<S>)
}
