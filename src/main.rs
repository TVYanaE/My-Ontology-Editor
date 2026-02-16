mod aliases;
mod modules;

use std::{
    sync::{
        Arc,
    },
};
use anyhow::Context;
use tracing::{instrument, error};
use winit::{
    event_loop::{ControlFlow}
};
use aliases::{
    WinitEventLoop
};
use modules::{
    app_dirs::{
        init_app_dirs,
    },
    logic_module::{
        LogicModule, 
    },
    graphics_module::{
        GraphicsModule,
        CustomEvent
    },
    logger::init_logger,
};

fn main() {
    let _guard = init_logger();
   
    std::panic::set_hook(Box::new(|panic_info|{
        error!("panic occured: {panic_info}"); 
    }));

    if let Err(err) = run() {
        error!(error = ?err, "application terminated");
    }
}

#[instrument(skip_all, err)]
fn run() -> anyhow::Result<()> {
    let event_loop = WinitEventLoop::<CustomEvent>::with_user_event()
        .build()
        .context("failed to create event loop")?;

    event_loop.set_control_flow(ControlFlow::Wait);

    // Theard Channels
    let custom_events = event_loop.create_proxy(); 
    

    let app_dirs = Arc::new(init_app_dirs()?);

    //  Logic theard 
    let logic_module_descriptor = LogicModule::init_logic_module(custom_events.clone(), app_dirs.clone());

    let mut graphics_module = GraphicsModule::new(app_dirs, logic_module_descriptor, custom_events); 

    event_loop.run_app(&mut graphics_module).context("event loop error exit")?;

    Ok(())
}


