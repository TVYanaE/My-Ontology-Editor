
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
    directory_path: PathBuf, 
}

pub struct CacheDirectory {
    directory_path: PathBuf
}

pub struct ApplicationDirectories {
    configuration_directory: ConfigurationDirectory,
    cache_directory: CacheDirectory,
}

#[cfg(debug_assertions)]
pub fn init_app_dirs() -> Result<ApplicationDirectories, InitAppDirsError> {
    let mut execute_path = env::current_exe()?;
    
    execute_path.pop();

    let mut config_directory_path = execute_path.clone();
    config_directory_path.push("config");

    let mut cache_directory_path = execute_path.clone();
    cache_directory_path.push("cache");

    let cache_directory = CacheDirectory {
        directory_path: cache_directory_path,
    };

    let configuration_directory = ConfigurationDirectory {
        directory_path: config_directory_path,
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

    let cache_directory = CacheDirectory {
        directory_path: cache_directory_path,
    };

    let configuration_directory = ConfigurationDirectory {
        directory_path: config_directory_path,
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
