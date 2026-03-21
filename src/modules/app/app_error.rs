use thiserror::Error;

use crate::modules::app::app_kernel::app_kernel_error::AppKernelError;
use crate::modules::app::gui::gui_error::GUIError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("App Kernel Error: {0}")]
    AppKernelError(#[from] AppKernelError),

    #[error("GUI Error: {0}")]
    GUIError(#[from] GUIError),
}
