mod modules;

use tracing::{instrument, error};
use thiserror::Error;
use eframe::{NativeOptions, run_native};

use modules::app::App;
use modules::app::app_dirs::{init_app_dirs, AppDirs};
use modules::logger::init_logger;


fn main() {
    let app_dirs = init_app_dirs()
        .expect("Application Directories initialisation error");
    let _guard = init_logger(&app_dirs.cache_directory.log_dir_path);
   
    std::panic::set_hook(Box::new(|panic_info|{
        error!("panic occured: {panic_info}"); 
    }));

    if let Err(err) = run(app_dirs) {
        error!(error = ?err, "application terminated");
    }
}

#[instrument(skip_all, err)]
fn run(app_dirs: AppDirs) -> Result<(), RunError> {  
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let native_options = NativeOptions::default();

    run_native(
        "My-Ontology-Editor", 
        native_options, 
        Box::new(|creation_context|{
            Ok(
                Box::new(
                    App::new(creation_context, app_dirs, runtime)
                )
            )
        })
    )?;

    Ok(())
}

#[derive(Debug, Error)]
enum RunError {
    #[error("Eframe Error: {0}")]
    EframeError(#[from] eframe::Error),

    #[error("STD IO Error")]
    STDIOError(#[from] std::io::Error),
}
