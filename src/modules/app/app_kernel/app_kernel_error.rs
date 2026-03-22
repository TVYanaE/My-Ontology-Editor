use thiserror::Error;

use crate::modules::app::app_kernel::app_event_handling::AppEventError;

#[derive(Debug, Error)]
pub enum AppKernelError {
    #[error("App Event Error: {0}")]
    AppEventError(#[from] AppEventError), 
}
