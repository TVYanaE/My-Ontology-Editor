mod confirmation_obtain_handling;

use tracing::instrument;

use crate::modules::app::AppKernel;
use crate::modules::app::confirmation_context::confirmation_context_manager::ConfirmationContextManager;

use crate::modules::app::gui::GUI;
use crate::modules::app::gui::gui_affect::GUIAffect;

use crate::modules::app::app_event::AppEvent;
use crate::modules::app::app_event::creating_project_event::CreatingProjectEvent;
use crate::modules::app::app_event::open_project_event::OpenProjectEvent;

use crate::modules::app::app_state::AppState;

use crate::modules::app::app_kernel::app_kernel_error::AppKernelError;

use self::confirmation_obtain_handling::confirmation_obtain_handling;

impl AppKernel {
    #[instrument(skip(ctx), err)]
    pub fn gui_affects_handling(
        gui_affect: GUIAffect,
        ctx: GUIAffectsHandlingContext, 
    ) -> Result<Option<AppEvent>, AppKernelError> {
        match ctx.current_app_state {
            AppState::Ready => {
                match gui_affect {
                    GUIAffect::ExitRequested => { 
                        Ok(
                            Some(
                                AppEvent::ShutdownReq
                            )
                        )
                    }, 

                    GUIAffect::CreateProjectInfo { 
                        project_name, 
                        project_path, 
                    } => {
                        Ok(
                            Some(
                                CreatingProjectEvent::CheckProjectInfo { 
                                    project_name, 
                                    project_path,
                                }.into()
                            )
                        ) 
                    },

                    GUIAffect::ConfirmationObtain { 
                        confirmation_type, 
                        decision 
                    } => {
                        confirmation_obtain_handling(
                            confirmation_type, 
                            decision, 
                            ctx.confirmation_context_manager
                        ) 
                    },

                    GUIAffect::OpenProjectInfo { 
                        project_file_path 
                    } => {
                        Ok(
                            Some(
                                OpenProjectEvent::CheckProjectInfo { 
                                    project_file_path, 
                                }.into() 
                            )
                        )
                    },
                }
            },

            AppState::Shutdown => {
                Ok(None)
            },
        }
    }
}

pub struct GUIAffectsHandlingContext<'c> {
    pub gui: &'c mut GUI,
    pub current_app_state: &'c AppState,
    pub confirmation_context_manager: &'c mut ConfirmationContextManager, 
}
