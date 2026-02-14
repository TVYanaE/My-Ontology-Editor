
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::InternalEvent,
                CustomEvents,
            },
            graphics_core::handle_redraw::RedrawError
        },
    },
};

pub fn handle_redraw_error(
    error: RedrawError,
    custom_events: &CustomEvents,    
) {
    match error {
        RedrawError::WGPUDataWasntFound => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        RedrawError::SurfaceError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        RedrawError::EGUIDataWasntFound => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
    }
}
