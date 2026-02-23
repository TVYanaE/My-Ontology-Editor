use std::{
    env,
    path::PathBuf,
};
#[allow(unused_imports)]
use directories::{ 
    ProjectDirs,
};
use thiserror::{
    Error
};

pub struct ConfigurationDirectory { 
    pub dir_path: PathBuf, 
}

pub struct CacheDirectory {
    pub dir_path: PathBuf,
    pub log_dir_path: PathBuf,
    pub projects_dir_path: PathBuf,
}

pub struct ApplicationDirectories {
    pub configuration_directory: ConfigurationDirectory,
    pub cache_directory: CacheDirectory,
}

#[cfg(debug_assertions)]
pub fn init_app_dirs() -> Result<ApplicationDirectories, InitAppDirsError> {
    let mut execute_path = env::current_exe()?;
    
    execute_path.pop();

    let mut config_directory_path = execute_path.clone();
    config_directory_path.push("config");

    if !config_directory_path.exists() {
        std::fs::create_dir_all(&config_directory_path)?; 
    }

    let mut cache_directory_path = execute_path.clone();
    cache_directory_path.push("cache");

    if !cache_directory_path.exists() {
        std::fs::create_dir_all(&cache_directory_path)?;
    } 

    let mut log_dir_path = cache_directory_path.clone();
    log_dir_path.push("logs");

    if !log_dir_path.exists() {
        std::fs::create_dir_all(&log_dir_path)?;
    } 

    let mut projects_dir_path = cache_directory_path.clone();
    projects_dir_path.push("projects");

    if !projects_dir_path.exists() {
        std::fs::create_dir_all(&projects_dir_path)?;
    }

    let cache_directory = CacheDirectory {
        dir_path: cache_directory_path,
        log_dir_path: log_dir_path,
        projects_dir_path: projects_dir_path,
    };

    let configuration_directory = ConfigurationDirectory {
        dir_path: config_directory_path,
    };

    let application_directories = ApplicationDirectories {
        configuration_directory: configuration_directory,
        cache_directory: cache_directory
    }; 

    return Ok(application_directories);
} 

 

#[cfg(not(debug_assertions))]
pub fn init_app_dirs(
) -> Result<ApplicationDirectories, InitAppDirsError> {
    let app_directory = ProjectDirs::from(
        "wfoojjaec.eu.org", 
        "wfoojjaec", 
        "My-Ontology-Editor")
    .ok_or_else(||{
        InitAppDirsError::ObtainingHomeDirPathError
    })?;

    let config_directory_path = app_directory.config_dir().to_path_buf();
    let cache_directory_path = app_directory.cache_dir().to_path_buf();
    
    let mut log_directory_path = app_directory.cache_dir().to_path_buf();
    log_directory_path.push("logs");

    let mut projects_dir_path = app_directory.cache_dir().to_path_buf();
    projects_dir_path.push("projects");

    if !config_directory_path.exists() {
        std::fs::create_dir_all(&config_directory_path)?; 
    }
    
    if !cache_directory_path.exists() {
        std::fs::create_dir_all(&cache_directory_path)?;
    }

    if !log_directory_path.exists() {
        std::fs::create_dir_all(&log_directory_path)?;
    }

    if !projects_dir_path.exists() {
        std::fs::create_dir_all(&projects_dir_path)?;
    }

    let cache_directory = CacheDirectory {
        dir_path: cache_directory_path,
        log_dir_path: log_directory_path,
        projects_dir_path: projects_dir_path,
    };

    let configuration_directory = ConfigurationDirectory {
        dir_path: config_directory_path,
    }; 

    let application_directories = ApplicationDirectories {
        configuration_directory: configuration_directory,
        cache_directory: cache_directory
    };

    return Ok(application_directories);
}

#[derive(Debug, Error)]
pub enum InitAppDirsError {
    #[error("No valid home directory path could be retrieved from the operating system.")]
    ObtainingHomeDirPathError,

    #[error("Getting Current Exe Path Error: {0}")]
    GettingCurrentExePathError(#[from] std::io::Error)
}
