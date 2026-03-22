
use crate::modules::app::app_state::AppState;
use crate::modules::app::app_error::AppError;
use crate::modules::app::app_kernel::AppKernelError;
use crate::modules::app::gui::gui_error::GUIError;

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
