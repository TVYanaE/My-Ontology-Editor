mod project_data;
mod project_error;
mod project_id;
mod project_logic;

use crate::{
    modules::{
        logic_module::{
            db_core::{
                DBConnectHandlerID,
            },
        }, 
    },
};
use self::{
    project_data::{
        ProjectData,
    },
};

pub use self::{
    project_data::{
        semantic_node::{
            SemanticNode, SemanticNodeID,
        },
    },
    project_id::ProjectID,
};

pub const PROJECT_EXTENSION: &str = "vontov";

pub struct Project {
    project_id: ProjectID,
    db_connect_handler_id: DBConnectHandlerID,
    project_data: ProjectData,
}

impl Project {
    pub fn new(
        project_id: ProjectID,
        db_connect_handler_id: DBConnectHandlerID,
    ) -> Self {
        Self { 
            project_id: project_id,
            db_connect_handler_id: db_connect_handler_id, 
            project_data: ProjectData::default(),
        }
    }

    pub fn get_project_id(&self) -> ProjectID {
        self.project_id.clone()
    }

    /// Upload all data from DB  
    pub fn sync_project_data_from_db(&mut self) {
         
    }

    /* pub fn get_all_semantic_nodes(&self) -> Vec<SemanticNode> {
             
    } */
}
