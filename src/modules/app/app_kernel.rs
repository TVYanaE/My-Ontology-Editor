mod app_event_handling;
mod app_kernel_error;
mod gui_affects_handling;

pub use self::app_event_handling::AppEventHandlingConxtex;
pub use self::gui_affects_handling::GUIAffectsHandlingContext;

pub use self::app_kernel_error::AppKernelError;
pub use self::app_event_handling::AppEventError;

pub struct AppKernel;
