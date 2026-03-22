pub mod projects_cache_error;

use std::io::Read;
use std::path::{Path, PathBuf};
use std::collections::hash_map::HashMap;

use crate::modules::consts::META_FILE_NAME;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_meta::ProjectMeta;

use self::projects_cache_error::ProjectsCacheError;

pub struct ProjectsCache {
    projects: HashMap<ProjectID, PathBuf>, 
    projects_cache_dir_path: PathBuf,
}

impl ProjectsCache {
    pub fn new<P>(
        project_dir_cache: &P
    ) -> Self 
    where 
        P: AsRef<Path>
    {
        Self { 
            projects: HashMap::with_capacity(64),
            projects_cache_dir_path: project_dir_cache.as_ref().to_path_buf()
        }
    } 

    pub fn add_project<P>(
        &mut self,
        project_id: &ProjectID,
        project_dir_cache: &P
    ) 
    where 
        P: AsRef<Path>
    { 
        self.projects.insert(
            project_id.clone(), 
            project_dir_cache.as_ref().to_path_buf()
        );
    }

    pub fn get_by_id(
        &self,
        project_id: &ProjectID,
    ) -> Option<PathBuf> {
        self.projects.get(project_id).cloned()
    }

    pub fn scan_cache_dir(&mut self) -> Result<(), ProjectsCacheError>  {
        println!("Loading projects from cache directory!");
        let projects_cache_dir = std::fs::read_dir(&self.projects_cache_dir_path)?; 

        for entry_res in projects_cache_dir {
            let entry = entry_res?; 
            if entry.file_type()?.is_dir() {
                let meta_file_path = entry.path().join(META_FILE_NAME); 
                let mut meta_file_handler = std::fs::File::open(meta_file_path)?; 
                
                let mut meta_buf: Vec<u8> = Vec::new(); 

                meta_file_handler.read_to_end(&mut meta_buf)?;

                let meta: ProjectMeta = toml::from_slice(&meta_buf)?;

                let project_id = ProjectID::from_str(&meta.project_id)?; 

                self.projects.insert(project_id, entry.path());
            }
        }

        Ok(())
    }
}
