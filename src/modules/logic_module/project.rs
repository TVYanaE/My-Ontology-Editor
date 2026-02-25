mod project_id;

use crate::{
    modules::{
        db_module::{
            DBConnectHandlerID,
        }, 
    },
};
use super::{
    project_view::{
        semantic_node::SemanticNode,
    },
};

pub use self::{
    project_id::ProjectID,
};

pub const PROJECT_EXTENSION: &str = "vontov";

pub struct Project {
    db_connect_handler_id: DBConnectHandlerID,
}

impl Project {
    pub fn new(
        db_connect_handler_id: DBConnectHandlerID,
    ) -> Self {
        Self { 
            db_connect_handler_id: db_connect_handler_id, 
        }
    }

    /* pub fn get_all_semantic_nodes(&self) -> Vec<SemanticNode> {
         
    } */
}
