use std::{
    path::{
        PathBuf,
    },
};

pub enum TaskContext {
    CreateProjectContext {
        project_name: String,
        project_path: PathBuf,
    }
}
