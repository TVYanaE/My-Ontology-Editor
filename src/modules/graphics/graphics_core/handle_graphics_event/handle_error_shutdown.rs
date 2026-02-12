use super::{
    handle_custom_event::CustomEventError,
    handle_window_event::WindowEventError,
};

pub enum ErrorShutdownContext {
    CustomEventError(CustomEventError),
    WindowEventError(WindowEventError),
}

impl From<CustomEventError> for ErrorShutdownContext {
    fn from(value: CustomEventError) -> Self {
        Self::CustomEventError(value)
    }
}
impl From<WindowEventError> for ErrorShutdownContext {
    fn from(value: WindowEventError) -> Self {
        Self::WindowEventError(value) 
    }
}


pub fn handle_error_shutdown(error_shutdown_context: ErrorShutdownContext) {

}
