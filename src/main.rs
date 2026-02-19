mod aliases;
mod modules;

use std::{
    sync::{
        Arc, RwLock,
    },
};
use anyhow::Context;
use tracing::{instrument, error};
use winit::{
    event_loop::{EventLoop, ControlFlow}
};
use modules::{
    app_dirs::{
        init_app_dirs, 
        ApplicationDirectories,
    },
    db_module::{
        DBModule,
    },
    logic_module::{
        LogicModule, 
    },
    graphics_module::{
        GraphicsModule,
        CustomEvent
    },
    logger::init_logger,
    shared::{
        project_manager::ProjectManager,
    },
};

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
fn run(app_dirs: ApplicationDirectories) -> anyhow::Result<()> {
    let event_loop = EventLoop::<CustomEvent>::with_user_event()
        .build()
        .context("failed to create event loop")?;

    event_loop.set_control_flow(ControlFlow::Wait);

    // Theard Channels
    let custom_events = event_loop.create_proxy();  
    
    let app_dirs = Arc::new(app_dirs);

    // DB module 
    let db_module_handler = DBModule::init_db_module();

    // Project Manager 
    let project_manager = Arc::new(
        RwLock::new(
            ProjectManager::new(
                db_module_handler.db_events.clone()
            )
        )
    );

    
    // Logic Module 
    let logic_module_descriptor = LogicModule::init_logic_module(
        custom_events.clone(), 
        app_dirs.clone(),
        project_manager,
        db_module_handler
    );

    // Graphics Module 
    let mut graphics_module = GraphicsModule::new(app_dirs, logic_module_descriptor, custom_events); 

    event_loop.run_app(&mut graphics_module).context("event loop error exit")?;

    Ok(())
}
