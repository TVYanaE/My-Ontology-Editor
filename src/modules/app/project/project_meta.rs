
use serde::{Serialize, Deserialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ProjectMeta {
    pub project_id: String, 
    pub project_name: String,
}
