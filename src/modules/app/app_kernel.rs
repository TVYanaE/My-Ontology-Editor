mod app_event_handling;
pub mod app_kernel_error;
mod gui_affects_handling;

pub use self::app_event_handling::AppEventHandlingConxtex;
pub use self::gui_affects_handling::GUIAffectsHandlingContext;

pub struct AppKernel;
