mod aliases;
mod modules;

use modules::{
    graphics::{
        events::graphics_event::ITCEvent,
        graphics_core::GraphicsCore,
    },
};
use winit::{
    event_loop::{EventLoop, ControlFlow}
};

fn main() {
    let event_loop = EventLoop::<ITCEvent>::with_user_event()
        .build()
        .unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    let _event_loop_proxy = event_loop.create_proxy();

    let mut application = GraphicsCore::default();

    event_loop.run_app(&mut application).unwrap();
}
