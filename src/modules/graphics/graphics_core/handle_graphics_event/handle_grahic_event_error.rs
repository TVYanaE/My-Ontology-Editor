use crate::{
    modules::{
        graphics::{
            events::{
                CustomEvents,
                graphics_event::{
                    CustomEvent, InternalEvent
                },
            },
        },
    },
};
use super::{
    GraphicsEventError,
    handle_external_event::ExternalEventError,
    handle_window_event::WindowEventError,
    handle_internal_event::InternalEventError
};



impl From<ExternalEventError> for GraphicsEventError {
    fn from(value: ExternalEventError) -> Self {
        Self::ExternalEventError(value)
    }    
}
impl From<WindowEventError> for GraphicsEventError {
    fn from(value: WindowEventError) -> Self {
        Self::WindowEventError(value) 
    }
}
impl From<InternalEventError> for GraphicsEventError {
    fn from(value: InternalEventError) -> Self {
        Self::InternalEventError(value)
    } 
}


pub fn handle_graphic_event_error(
    graphics_event_error: GraphicsEventError,
    custom_events: &CustomEvents 
) {
    match graphics_event_error {
        GraphicsEventError::ExternalEventError(error) => {
            match error {

            }
        }, 
        GraphicsEventError::WindowEventError(error) => {
            match error {
                WindowEventError::SurfaceError(_) => { 
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                WindowEventError::WGPUDataWasntFound => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                WindowEventError::EGUIDataWasntFound => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
        GraphicsEventError::InternalEventError(error) => {
            match error {
                InternalEventError::RequestDeviceError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                InternalEventError::CreateSurfaceError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                InternalEventError::RequestAdapterError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                InternalEventError::TextureFormatIsntSupported => { 
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                InternalEventError::SendError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                InternalEventError::LogicThreadWasntFound => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
    }
}
