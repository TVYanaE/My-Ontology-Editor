use std::{
    path::PathBuf
};

pub struct ProjectMainFilesLayout {
    pub project_meta_file: ProjectMetaFile,
}

impl ProjectMainFilesLayout {
    pub fn create_default_main_files_layout(
    ) -> ProjectMainFilesLayout {
        let project_meta_file = ProjectMetaFile::create();

        Self { 
            project_meta_file: project_meta_file 
        }
    } 
}

pub struct ProjectMetaFile {
    pub path: PathBuf,
}

impl ProjectMetaFile {
    fn create(
        
    ) -> Self {
        let mut path = PathBuf::new();
        path.set_file_name("meta");
        path.add_extension("toml");

        Self { 
            path: path,
        }
    }
}


