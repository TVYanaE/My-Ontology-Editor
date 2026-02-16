mod aliases;
mod modules;

use std::{
    sync::{
        Arc,
    },
};
use calloop::{
    channel::{
        channel,
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
        events::LogicEvent,
        logic_core::init_logic, 
    },
    graphics_module::{
        GraphicsModule,
        CustomEvent
    },
    logger::init_logger,
    shared::LogicThreadDescriptor,
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
    let event_loop_proxy = event_loop.create_proxy(); 
    let (sender, channel) = channel::<LogicEvent>();

    let app_dirs = Arc::new(init_app_dirs()?);

    //  Logic theard 
    let handle = init_logic(event_loop_proxy.clone(), app_dirs.clone(), channel);

    let logic_thread_descriptor = LogicThreadDescriptor { thread_handle: Some(handle), sender };

    let mut graphics_module = GraphicsModule::new(app_dirs, logic_thread_descriptor, event_loop_proxy); 

    event_loop.run_app(&mut graphics_module).context("event loop error exit")?;

    Ok(())
}


