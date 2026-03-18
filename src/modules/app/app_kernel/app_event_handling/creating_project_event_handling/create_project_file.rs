use std::sync::Arc;
use std::path::PathBuf;

use tokio::fs::DirBuilder;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

use tokio_tar::Builder as TarBuilder;

use thiserror::Error;

use super::super::super::super::app_event::AppEvent;
use super::super::super::super::app_dirs::AppDirs;
use super::super::super::super::project::project_id::ProjectID;
use super::super::super::super::project::project_meta::ProjectMeta;

use super::super::super::super::super::migrations::{
    SEMANTIC_NODES_TABLE_MIGRATION,
    SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION,
    FILES_TABLE_MIGRATION,
};

#[derive(Debug, Error)]
pub enum CreateProjectFileError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("SQLX Error: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[error("Toml Crate Error: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("Strip Prefix Error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),
}


pub async fn create_project_file(
    project_name: String, 
    project_path: String,
    app_dirs: Arc<AppDirs>,
) -> Result<(), CreateProjectFileError> {
    let mut project_dir_cache = app_dirs.cache_directory.projects_dir_path.clone();

    let project_id = ProjectID::new();
    let project_id_str = project_id.get_str();

    project_dir_cache.push(&project_id_str); 

    if tokio::fs::try_exists(&project_dir_cache).await? {
        tokio::fs::remove_dir_all(&project_dir_cache).await?; 
    }

    let dir_builder = DirBuilder::new();

    dir_builder.create(&project_dir_cache).await?;

    // Creating Meta File
    let project_meta = ProjectMeta {
        project_id: project_id_str,
        project_name: project_name.clone(),
    };

    let project_meta_data = toml::to_string(&project_meta)?;

    let mut meta_file_path = project_dir_cache.clone();
    meta_file_path.push("meta");
    meta_file_path.set_extension("toml");

    let mut meta_file_handler = TokioFile::create_new(meta_file_path).await?; 
    meta_file_handler.write_all(project_meta_data.as_bytes()).await?;

    // Creating Semantic Node dir
    let mut semantic_nodes_dir = project_dir_cache.clone();
    semantic_nodes_dir.push("semantic_nodes");

    dir_builder.create(semantic_nodes_dir).await?;

    // Creating Project DB dir and DB file
    let mut db_dir = project_dir_cache.clone();
    db_dir.push("db");
   
    dir_builder.create(&db_dir).await?; 

    let mut db_file_path = db_dir.clone();
    db_file_path.push("db.sqlite");

    let db_url = format!("sqlite://{}", db_file_path.to_str().unwrap());

    println!("Path to db: {}", db_url);

    sqlx::Sqlite::create_database(&db_url).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
   
    // Migrations 
    sqlx::query(SEMANTIC_NODES_TABLE_MIGRATION).execute(&pool).await?;
    sqlx::query(SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION).execute(&pool).await?;
    sqlx::query(FILES_TABLE_MIGRATION).execute(&pool).await?;

    // Creating Project File 
    let mut project_file_path = PathBuf::new(); 
    project_file_path.push(project_path);
    project_file_path.push(project_name);
    project_file_path.set_extension("vontov");

    if project_file_path.exists() {
        if project_file_path.is_file() {
            tokio::fs::remove_file(&project_file_path).await?; 
        }
        // TODO: Logic for case If in selected dir exsist folder with project_name.vontov
    }
    let project_file_handler = TokioFile::create_new(project_file_path).await?;

    let mut tar_builder = TarBuilder::new(project_file_handler); 

    tar_builder.append_dir_all("", project_dir_cache).await?;

    Ok(())
}



// Callback
pub fn create_project_file_callback(
 
) -> Option<AppEvent> {

    None
}
