mod project_topology;

use std::{
    path::{PathBuf, Path},
}; 
use thiserror::{
    Error,
};
use self::{
    project_topology::{
        ProjectTopology,
        ProjectTopologyError,
    },
};

/// Descriptor of unpacked project
pub struct Project {
    project_dirs_map: ProjectDirsMap,
    project_topology: ProjectTopology,
}

impl Project {
    pub fn new(
        project_root_path: &impl AsRef<Path>,
        semantic_nodes_dir_path: &impl AsRef<Path>,
        project_meta_file_path: &impl AsRef<Path>,
    ) -> Result<Self, ProjectError> {
        let project_dirs_map = ProjectDirsMap {
            semantic_nodes_dir_path: semantic_nodes_dir_path.as_ref().to_path_buf(),
            project_meta_file_path: project_meta_file_path.as_ref().to_path_buf(),
        };

        let project_topology = ProjectTopology::new(project_root_path)?;

        Ok(Self { 
            project_dirs_map: project_dirs_map, 
            project_topology: project_topology,
        })

    }
}

struct ProjectDirsMap {
    pub semantic_nodes_dir_path: PathBuf,
    pub project_meta_file_path: PathBuf,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Project Topology Error: {0}")]
    ProjectTopologyError(#[from] ProjectTopologyError)
}
