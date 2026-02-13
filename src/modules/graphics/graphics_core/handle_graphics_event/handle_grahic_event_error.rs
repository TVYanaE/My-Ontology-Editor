use crate::{
    modules::{
        graphics::{
            events::{
                CustomEvents,
                graphics_event::{
                    CustomEvent
                },
            },
        },
    },
};
use super::{
    handle_custom_event::CustomEventError,
    handle_window_event::WindowEventError,
};

pub enum GraphicsEventError {
    CustomEventError(CustomEventError),
    WindowEventError(WindowEventError),
}

impl From<CustomEventError> for GraphicsEventError {
    fn from(value: CustomEventError) -> Self {
        Self::CustomEventError(value)
    }
}
impl From<WindowEventError> for GraphicsEventError {
    fn from(value: WindowEventError) -> Self {
        Self::WindowEventError(value) 
    }
}


pub fn handle_graphic_event_error(
    graphics_event_error: GraphicsEventError,
    custom_events: &CustomEvents 
) {
    match graphics_event_error {
        GraphicsEventError::CustomEventError(error) => {
            match error {
                CustomEventError::RequestDeviceError(_) => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                CustomEventError::CreateSurfaceError(_) => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                CustomEventError::RequestAdapterError(_) => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                CustomEventError::TextureFormatIsntSupported => { 
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                CustomEventError::SendError(_) => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                CustomEventError::LogicThreadWasntFound => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
        GraphicsEventError::WindowEventError(error) => {
            match error {
                WindowEventError::SurfaceError(_) => { 
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                WindowEventError::WGPUDataWasntFound => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                WindowEventError::EGUIDataWasntFound => {
                    custom_events
                        .send_event(CustomEvent::AppShutdownReq)
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
    }
}
