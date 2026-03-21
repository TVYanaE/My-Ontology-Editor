use thiserror::Error;

use crate::modules::app::app_kernel::app_event_handling::{
    app_event_handling_error::AppEventHandlingError
};

#[derive(Debug, Error)]
pub enum AppKernelError {
    #[error("App Event Handling Error: {0}")]
    AppEventHandlingError(#[from]AppEventHandlingError), 
}
