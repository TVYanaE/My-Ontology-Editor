use thiserror::Error;

use crate::modules::app::project::project_error::ProjectError;

#[derive(Debug, Error)]
pub enum OpenProjectEventError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),
 
    #[error("Bytemuck Pod Cast Error: {0}")]
    BytemuckPodCastError(bytemuck::PodCastError),

    #[error("Project Error: {0}")]
    ProjectError(#[from] ProjectError),
}
