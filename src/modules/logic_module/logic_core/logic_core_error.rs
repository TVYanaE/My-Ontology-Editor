use thiserror::{
    Error
};
use super::{
    super::{
        infrastructure::{
            logic_module_io::{
                event_sender::EventSender,
                event_manager::EventManagerError,
            },
            job_manager::JobManagerError,
        },
        project_manager::ProjectManagerError,
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
