use std::sync::Arc;
use std::path::PathBuf;

use tokio::fs::DirBuilder;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePoolOptions;

use tokio_tar::Builder as TarBuilder;

use thiserror::Error;

use crate::modules::consts::{
    PROJECT_FILE_EXTENSION,
    CURRENT_PROJECT_FILE_VERSION,
    META_FILE_NAME,
};

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::creating_project_event::CreatingProjectEvent;

use crate::modules::app::app_dirs::AppDirs;

use crate::modules::app::project::project_id::ProjectID;
use crate::modules::app::project::project_meta::ProjectMeta;
use crate::modules::app::project::project_file_header::ProjectFileHeader;

use crate::modules::migrations::{
    SEMANTIC_NODES_TABLE_MIGRATION,
    SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION,
    FILES_TABLE_MIGRATION,
};

use super::CreatingProjectEventError;

use super::super::app_event_error::AppEventError;

#[derive(Debug, Error)]
pub enum CreateProjectFileError {
    #[error("STD IO Error: {0}")]
    STDIOError(#[from] std::io::Error),

    #[error("SQLX Error: {0}")]
    SQLXError(#[from] sqlx::Error),

    #[error("Toml Crate Error: {0}")]
    TomlSerError(#[from] toml::ser::Error),

    #[error("Strip Prefix Error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),  
}

pub struct CreateProjectFileContext {
    project_id: ProjectID,
    project_name: String,
    project_dir_cache: PathBuf,
}

pub async fn create_project_file(
    project_name: String, 
    project_path: String,
    app_dirs: Arc<AppDirs>,
) -> Result<CreateProjectFileContext, CreateProjectFileError> {
    let project_id = ProjectID::new();
    let project_id_str = project_id.get_str();

    let project_dir_cache = app_dirs.cache_directory.projects_dir_path.join(&project_id_str); 

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

    let meta_file_path = project_dir_cache.join(META_FILE_NAME);

    let mut meta_file_handler = TokioFile::create_new(meta_file_path).await?; 
    meta_file_handler.write_all(project_meta_data.as_bytes()).await?;

    // Creating Semantic Node dir
    let semantic_nodes_dir = project_dir_cache.join("semantic_nodes");

    dir_builder.create(semantic_nodes_dir).await?;

    // Creating Project DB dir and DB file
    let db_dir = project_dir_cache.join("db");
   
    dir_builder.create(&db_dir).await?; 

    let db_file_path = db_dir.join("db.sqlite");

    let db_url = format!("sqlite://{}", db_file_path.to_str().unwrap());

    sqlx::Sqlite::create_database(&db_url).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await?;
   
    // Migrations 
    sqlx::query(SEMANTIC_NODES_TABLE_MIGRATION).execute(&pool).await?;
    sqlx::query(SEMANTIC_NODES_RELATIONS_TABLE_MIGRATION).execute(&pool).await?;
    sqlx::query(FILES_TABLE_MIGRATION).execute(&pool).await?;

    // Creating Project File 
    let mut project_file_path = PathBuf::new(); 
    project_file_path.push(&project_path);
    project_file_path.push(&project_name);
    project_file_path.set_extension(PROJECT_FILE_EXTENSION);

    if project_file_path.exists() {
        if project_file_path.is_file() {
            tokio::fs::remove_file(&project_file_path).await?; 
        }
        // TODO: Logic for case If in selected dir exsist folder with project_name.vontov
    };

    let mut project_file_handler = TokioFile::create_new(&project_file_path).await?;

    // Adding Header into Project File 
    let project_file_header = ProjectFileHeader::new(
        CURRENT_PROJECT_FILE_VERSION,
        &project_id
    );

    project_file_handler.write_all(project_file_header.as_bytes()).await?;

    let mut tar_builder = TarBuilder::new(project_file_handler); 

    tar_builder.append_dir_all("", &project_dir_cache).await?;

    Ok(
        CreateProjectFileContext { 
            project_id,
            project_name,
            project_dir_cache,
        }
    )
}


// Callback
pub fn create_project_file_callback(
    res: Result<CreateProjectFileContext, CreateProjectFileError> 
) -> Option<AppEvent> {
    match res {
        Ok(context) => {
            Some(
                AppEvent::CreatingProjectEvent(
                    CreatingProjectEvent::ProjectFileCreated { 
                        project_id: context.project_id, 
                        project_dir_cache: context.project_dir_cache,
                    }
                )
            )
        },
        Err(error) => {
            match error {
                CreateProjectFileError::STDIOError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::CreatingProjectEventError(
                                CreatingProjectEventError::STDIO(error)
                            )
                        )
                    )
                },
                CreateProjectFileError::SQLXError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::CreatingProjectEventError(
                                CreatingProjectEventError::Sqlx(error)
                            )
                        )
                    )
                },
                CreateProjectFileError::TomlSerError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::CreatingProjectEventError(
                                CreatingProjectEventError::TomlSer(error)
                            )
                        )
                    )
                },
                CreateProjectFileError::StripPrefixError(error) => {
                    Some(
                        AppEvent::AppEventError(
                            AppEventError::CreatingProjectEventError(
                                CreatingProjectEventError::StripPrefix(error)
                            )
                        ) 
                    )
                },
            } 
        },
    }
}
