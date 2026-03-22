pub mod project_error;
pub mod project_file_header;
pub mod project_id;
pub mod project_manager;
pub mod project_meta;
pub mod projects_cache;

use std::path::{Path, PathBuf};

use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use sqlx::Pool;
use sqlx::sqlite::{Sqlite, SqlitePoolOptions};

use crate::modules::consts::META_FILE_NAME;

use self::project_error::ProjectError;
use self::project_id::ProjectID;
use self::project_meta::ProjectMeta;

pub struct Project {
    project_id: ProjectID,
    project_name: String,
    db_pool: Pool<Sqlite>,
    project_dir_path: PathBuf,
}

impl Project {
    pub async fn new(
        project_dir_cache: impl AsRef<Path>, 
    ) -> Result<Self, ProjectError> {
        let meta_file_path = project_dir_cache.as_ref().join(META_FILE_NAME); 

        let mut meta_file_handler = TokioFile::open(meta_file_path).await?; 

        let mut meta_file_str = String::with_capacity(64);

        meta_file_handler.read_to_string(&mut meta_file_str).await?;

        let project_meta: ProjectMeta = toml::from_str(&meta_file_str)?;
        
        let project_id = ProjectID::from_str(&project_meta.project_id)?;

        let db_file_path = project_dir_cache.as_ref().join("db/db.sqlite");
 
        let db_url = format!("sqlite://{}", db_file_path.to_str().unwrap());

        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(
            Self { 
                project_id,
                project_name: project_meta.project_name,
                db_pool,
                project_dir_path: project_dir_cache.as_ref().to_path_buf(),
            }
        )
    }

    pub fn get_project_name(&self) -> &str {
        &self.project_name
    }

    /* pub fn get_project_id(&self) -> &ProjectID {
        &self.project_id
    } */
}
