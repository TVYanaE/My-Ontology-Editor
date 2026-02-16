use super::{
    CustomEvents,
    graphics_event::InternalEvent,
    GraphicsEventError,
    ExternalEventError,
    WindowEventError,
    InternalEventError,
    RedrawEventError,
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
impl From<RedrawEventError> for GraphicsEventError {
    fn from(value: RedrawEventError) -> Self {
        Self::RedrawEventError(value)
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
                WindowEventError::WGPUBackendError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
                WindowEventError::EGUIBackendError(_) => {
                    custom_events
                    .send_event(InternalEvent::AppShutdownReq.into())
                    .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
        GraphicsEventError::InternalEventError(error) => {
            match error {
                InternalEventError::WGPUBackendError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed."); 
                },
                InternalEventError::MPSCChannelError(_) => {
                    custom_events
                        .send_event(InternalEvent::AppShutdownReq.into())
                        .expect("Critical Error.Event Loop Proxy has been closed.");
                },
            }
        },
        GraphicsEventError::RedrawEventError(_error) => {
            custom_events
                .send_event(InternalEvent::AppShutdownReq.into())
                .expect("Critical Error.Event Loop Proxy has been closed.");
        },
    }
}
