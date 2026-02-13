mod aliases;
mod modules;

use std::{
    sync::Arc,
};
use flume::{
    unbounded,
};
use anyhow::Context;
use tracing::{instrument, error};
use winit::{
    event_loop::{EventLoop, ControlFlow}
};
use modules::{
    app_dirs::{
        init_app_dirs,
    },
    logic::{
        events::LogicEvent,
        logic_core::LogicCore,
    },
    graphics::{
        events::graphics_event::CustomEvent,
        graphics_core::GraphicsCore,
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
    let event_loop = EventLoop::<CustomEvent>::with_user_event()
        .build()
        .context("failed to create event loop")?;

    event_loop.set_control_flow(ControlFlow::Wait);

    // Theard Channels
    let event_loop_proxy = event_loop.create_proxy();
    let (
        logic_event_channel_sender,
        logic_event_channel_receiver
    ) = unbounded::<LogicEvent>();


    let app_dirs = Arc::new(init_app_dirs()?);

    let logic_core = LogicCore::start(
        event_loop_proxy.clone(), 
        logic_event_channel_sender,
        logic_event_channel_receiver, 
        app_dirs.clone()
    );

    let mut application = GraphicsCore::new(
        event_loop_proxy, 
        app_dirs,
        logic_core
    );

    event_loop.run_app(&mut application).context("event loop error exit")?;

    Ok(())
} 
