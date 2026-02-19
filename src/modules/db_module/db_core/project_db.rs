mod project_db_logic;

use std::{
    path::{
        Path,
    },
};
use thiserror::{
    Error,
};
use rusqlite::{
    Connection,
};
use oneshot::{
    Sender,
};
use self::{
    project_db_logic::ProjectDBLogic,
};

enum ProjectDBState {
    Connected(Connection),
    NotConnected,
    Processing,
}

impl Default for ProjectDBState {
    fn default() -> Self {
        Self::NotConnected
    }
}

#[derive(Debug, Error)]
pub enum ProjectDBError {
    #[error("Rusqlite Error: {0}")]
    RusQliteError(#[from] rusqlite::Error),

    #[error("OneShot Send Error: {0}")]
    OneShotSendError(#[from] oneshot::SendError<Result<(), ProjectDBError>>),
}

pub struct ProjectDB {
    state: ProjectDBState, 
}


impl ProjectDB {
    pub fn new() -> Self {
        Self { 
            state: ProjectDBState::default() 
        }
    }

    pub fn close_connection(&mut self) {
        let current_state = std::mem::replace(
            &mut self.state, 
            ProjectDBState::Processing
        );

        self.state = match current_state {
            ProjectDBState::Connected(_) => {
                // logic for graceful shutdown of connection
                ProjectDBState::NotConnected
            },
            current_state => current_state, 
        }
    } 

    pub fn open_connection(
        &mut self,
        project_root_path: &impl AsRef<Path>,
        response_target: Sender<Result<(), ProjectDBError>>
    ) -> Result<(), ProjectDBError> {
        match ProjectDBLogic::open_connection_handle(
            project_root_path,
            response_target,
        ) {
            Ok(connection) => {
                self.state = ProjectDBState::Connected(connection);
                Ok(())
            },
            Err(error) => {
                Err(error)
            },
        }
    }
}
