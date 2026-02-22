/* use uuid::{Uuid};
use serde::{
    Serialize, Deserialize,
};


pub struct MainFilesPayloadsDescriptor<'c> {
    pub project_id: Uuid,
    pub project_name: &'c str,
}

pub struct ProjectMainFilesPayloads {
    pub meta_file: ProjectMetaFilePayload, 
}

impl ProjectMainFilesPayloads {
    pub fn create(
        descriptor: MainFilesPayloadsDescriptor
    ) -> Result<Self, ProjectManagerError> {
        let meta_file = ProjectMetaFilePayload::create(
            descriptor.project_id, 
            descriptor.project_name
        )?;

        Ok(Self { 
            meta_file: meta_file 
        })
    }
}

pub struct ProjectMetaFilePayload {
    pub data: String
}   

impl ProjectMetaFilePayload {
    fn create(
        project_id: Uuid,
        project_name: &str,
    ) -> Result<Self, ProjectManagerError> {
        let data = toml::to_string(&ProjectMetaFileData {
            project_name: project_name.to_string(),
            project_id: project_id
        })?;

        Ok(Self { 
            data: data 
        })
    }
}

#[derive(Serialize, Deserialize)]
struct ProjectMetaFileData {
    project_id: Uuid,
    project_name: String,
} */
