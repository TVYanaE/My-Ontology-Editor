use super::{
    super::{
        events::{CustomEvents, InternalEvent},
    },
    graphic_event_error::GraphicsEventError, 
};


pub fn graphic_event_error_handle(
    graphics_event_error: GraphicsEventError,
    custom_events: &CustomEvents 
) { 
    match graphics_event_error {
        GraphicsEventError::SurfaceError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        GraphicsEventError::WGPUBackendError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        GraphicsEventError::MPSCChannelError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        GraphicsEventError::EGUIBackendError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
        GraphicsEventError::UIError(_) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
    }
    
}
