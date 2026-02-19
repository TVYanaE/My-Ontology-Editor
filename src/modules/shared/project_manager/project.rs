
use std::{
    path::{PathBuf, Path},
}; 
use thiserror::{
    Error,
};
use oneshot::{
    channel as one_shot_channel,
};
use crate::{
    aliases::{
        DBEvents,
    },
    modules::{
        db_module::{DBEvent, ProjectDBError}
    },
};


/// Descriptor of unpacked project
pub struct Project {
    project_dirs_map: ProjectDirsMap,
    db_events: DBEvents,
}

impl Project {
    pub fn new(
        project_root_path: &impl AsRef<Path>,
        semantic_nodes_dir_path: &impl AsRef<Path>,
        project_meta_file_path: &impl AsRef<Path>,
        db_events: DBEvents,
    ) -> Result<Self, ProjectError> {
        let project_dirs_map = ProjectDirsMap {
            semantic_nodes_dir_path: semantic_nodes_dir_path.as_ref().to_path_buf(),
            project_meta_file_path: project_meta_file_path.as_ref().to_path_buf(),
        };
       
        let project_root_path_own = project_root_path.as_ref().to_path_buf();

        // Oneshot channel for respone
        let (
            sender, 
            recevier
        ) = one_shot_channel::<Result<(), ProjectDBError>>();

        db_events.send(DBEvent::OpenConnection{
            response_target: sender,
            project_root_path: project_root_path_own,
        })?;
       
        recevier.recv()??;

        Ok(Self { 
            project_dirs_map: project_dirs_map, 
            db_events: db_events,
        })

    }
}

struct ProjectDirsMap {
    pub semantic_nodes_dir_path: PathBuf,
    pub project_meta_file_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("MPSC Channel was closed {0}")]
    MPSCChannelDBEventError(#[from] std::sync::mpsc::SendError<DBEvent>), 

    #[error("One Shot Recv Error: {0}")]
    OneShotRecvError(#[from] oneshot::RecvError),

    #[error("Project DB Error: {0} ")]
    ProjectDBError(#[from] ProjectDBError),
}
