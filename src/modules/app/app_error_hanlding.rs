
use super::app_state::AppState;
use super::app_error::AppError;
use super::app_kernel::app_kernel_error::AppKernelError;
use super::gui::gui_error::GUIError;

pub fn app_error_handling(
    error: AppError,
) -> Option<AppState> {
    match error {
        AppError::AppKernelError(error) => {
            app_kernel_error_handling(error)
        },
        AppError::GUIError(error) => {
            gui_error_handling(error)
        },
    }
}


fn app_kernel_error_handling(
    error: AppKernelError 
) -> Option<AppState> {
    Some(AppState::Shutdown)
}

fn gui_error_handling(
    error: GUIError,
) -> Option<AppState> {
    Some(AppState::Shutdown)
}
