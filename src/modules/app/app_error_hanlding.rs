use crate::modules::app::app_state::AppState;
use crate::modules::app::app_error::AppError;
use crate::modules::app::app_kernel::{
    AppKernelError, AppEventError,
    CreatingProjectEventError, InitialisationEventError,
    OpenProjectEventError,
};
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
    match error {
        AppKernelError::AppEventError(error) => {
            app_event_error_handling(error) 
        },
    } 
}

fn app_event_error_handling(
    error: AppEventError 
) -> Option<AppState> {
    match error {
        AppEventError::CreatingProjectEventError(error) => {
            creating_project_event_error_handling(error)
        },
        AppEventError::InitialisationEventError(error) => {
            initialisation_event_error_handling(error)
        },
        AppEventError::OpenProjectEventError(error) => {
            open_project_event_error(error)
        },
    }
}

// TODO!: Logic for handling Error
fn creating_project_event_error_handling(
    error: CreatingProjectEventError,
) -> Option<AppState> {
    match error {
        CreatingProjectEventError::STDIO(_error) => {},
        CreatingProjectEventError::Sqlx(_error) => {},
        CreatingProjectEventError::TomlSer(_error) => {},
        CreatingProjectEventError::StripPrefix(_error) => {},
        CreatingProjectEventError::ProjectError(_error) => {},
    } 
    Some(AppState::Shutdown)
}

// TODO!: Logic for handling Error
fn initialisation_event_error_handling(
    error: InitialisationEventError
) -> Option<AppState> {
    match error {
        InitialisationEventError::STDError(_error) => {},
        InitialisationEventError::UuidError(_error) => {},
        InitialisationEventError::TOMLDesError(_error) => {},
    }
    Some(AppState::Shutdown)
}

// TODO!: Logic for handling Error
fn open_project_event_error(
    error: OpenProjectEventError
) -> Option<AppState> {
    match error {
        OpenProjectEventError::STDIOError(_error) => {},
        OpenProjectEventError::BytemuckPodCastError(_error) => {},
        OpenProjectEventError::ProjectError(_error) => {},
    }
    Some(AppState::Shutdown)
}

fn gui_error_handling(
    _error: GUIError,
) -> Option<AppState> {
    Some(AppState::Shutdown)
}
