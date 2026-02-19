use std::{
    path::{Path},
};
use thiserror::{
    Error,
};

pub struct ProjectTopology {
    // handler of DB Thread db_connection: RusQliteConnection,
}

impl ProjectTopology {
    pub fn new(
        project_root_path: &impl AsRef<Path>
    ) -> Result<Self, ProjectTopologyError> {
        let mut db_path = project_root_path.as_ref().to_path_buf();
        db_path.push("topology");
        db_path.set_extension("db3");


        // TODO: Logic for migration

        Ok(Self { 
        })
    }
}

#[derive(Debug, Error)]
pub enum ProjectTopologyError {
    #[error("Rusqlite Error: {0}")]
    RusQliteError(#[from] rusqlite::Error)
}
