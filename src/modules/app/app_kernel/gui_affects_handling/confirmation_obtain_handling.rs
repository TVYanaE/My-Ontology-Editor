use super::super::app_kernel_error::AppKernelError;
use super::super::super::app_event::AppEvent;
use super::super::super::app_event::creating_project_event::CreatingProjectEvent;
use super::super::super::confirmation_context::ConfirmationContext;
use super::super::super::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use super::super::super::gui::gui_command::ConfirmationType;


pub fn confirmation_obtain_handling(
    confirmation_type: ConfirmationType, 
    decision: bool,
    confirmation_context_manager: &mut ConfirmationContextManager,
) -> Result<Option<AppEvent>, AppKernelError> {
    match confirmation_type {
        ConfirmationType::OverwriteProjectFile(
            confirmation_context_id
        ) => {
            if let Some(context) = 
                confirmation_context_manager
                .remove(&confirmation_context_id) {
                match context {
                    ConfirmationContext::OverwriteProjectFileContext { 
                        project_name, 
                        project_path 
                    } => {
                        if decision {
                            Ok(
                                Some(
                                    CreatingProjectEvent::CreateProject { 
                                        project_name, 
                                        project_path 
                                    }.into()
                                )
                            ) 
                        }
                        else {
                            Ok(None)
                        }
                    },
                    _ => {
                        // TODO: Logic for Wrong Context
                        Ok(None)
                    },
                }
            }else {
                // TODO: Logic for situation when Confirmation context wasn't found
                Ok(None)
            }
        },
    }
}
