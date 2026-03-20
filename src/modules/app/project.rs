pub mod project_cache;
pub mod project_error;
pub mod project_file_header;
pub mod project_id;
pub mod project_meta;

use std::path::Path;

use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;

use sqlx::Pool;
use sqlx::sqlite::{Sqlite, SqlitePoolOptions};

use self::project_error::ProjectError;
use self::project_id::ProjectID;
use self::project_meta::ProjectMeta;

pub struct Project {
    project_id: ProjectID,
    project_name: String,
    db_pool: Pool<Sqlite>,
}

impl Project {
    pub async fn new(
        project_dir_cache: &Path, 
    ) -> Result<Self, ProjectError> {
        let mut meta_file_path = project_dir_cache.to_path_buf(); 
        meta_file_path.push("meta");
        meta_file_path.set_extension("toml");

        let mut meta_file_handler = TokioFile::open(meta_file_path).await?; 

        let mut meta_file_str = String::with_capacity(64);

        meta_file_handler.read_to_string(&mut meta_file_str).await?;

        let project_meta: ProjectMeta = toml::from_str(&meta_file_str)?;
        
        let project_id = ProjectID::from_str(&project_meta.project_id)?;

        let mut db_file_path = project_dir_cache.to_path_buf();
        db_file_path.push("db");
        db_file_path.push("db");
        db_file_path.set_extension("sqlite");
 
        let db_url = format!("sqlite://{}", db_file_path.to_str().unwrap());

        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        Ok(
            Self { 
                project_id: project_id,
                project_name: project_meta.project_name,
                db_pool: db_pool 
            }
        )
    }

    pub fn get_project_name(&self) -> &str {
        &self.project_name
    }

    pub fn get_project_id(&self) -> &ProjectID {
        &self.project_id
    }
}
