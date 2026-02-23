use std::{
    path::PathBuf
};
use serde::{
    Serialize, Deserialize,
};
use super::{
    super::{
        super::{
            super::{
                project::ProjectID,
            },
        },
    },
};


#[derive(Debug)]
pub struct ProjectMainFilesLayout {
    pub project_meta_file: ProjectMetaFile,
    pub project_db_file: ProjectDBFile,
}

impl Default for ProjectMainFilesLayout {
    fn default() -> Self {
        Self { 
            project_meta_file: ProjectMetaFile::default(),
            project_db_file: ProjectDBFile::default(),
        } 
    } 
}

#[derive(Debug)]
pub struct ProjectMetaFile {
    pub path: PathBuf,
}

impl Default for ProjectMetaFile {
    fn default() -> Self {
        let mut path = PathBuf::new();
        path.set_file_name("meta");
        path.add_extension("toml");

        Self { 
            path: path,
        } 
    } 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectMetaFileData {
    pub project_id: ProjectID,
    pub project_name: String,
}

#[derive(Debug)]
pub struct ProjectDBFile {
    pub path: PathBuf
}

impl Default for ProjectDBFile {
    fn default() -> Self {
        let mut path = PathBuf::new();
        path.push("project_db");
        path.add_extension("db3");
        
        Self {
            path: path,
        } 
    } 
}
