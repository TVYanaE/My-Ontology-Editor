mod confirmation_obtain_handling;

use tracing::instrument;

use super::AppKernel;
use super::super::confirmation_context::confirmation_context_manager::ConfirmationContextManager;
use super::super::gui::GUI;
use super::super::gui::gui_affect::GUIAffect;
use super::super::gui::gui_command::GUICommand;
use super::super::app_event::AppEvent;
use super::super::app_event::creating_project_event::CreatingProjectEvent;
use super::super::app_state::AppState;
use super::app_kernel_error::AppKernelError;

use self::confirmation_obtain_handling::confirmation_obtain_handling;

impl AppKernel {
    #[instrument(
        skip(
            self, gui, confirmation_context_manager
        ), 
        err
    )]
    pub fn gui_affects_handling(
        &self, 
        gui_affect: GUIAffect,
        gui: &mut GUI,
        current_app_state: &AppState,
        confirmation_context_manager: &mut ConfirmationContextManager,
    ) -> Result<Option<AppEvent>, AppKernelError> {
        match current_app_state {
            AppState::Ready => {
                match gui_affect {
                    GUIAffect::ExitRequested => { 
                        Ok(
                            Some(
                                AppEvent::ShutdownReq
                            )
                        )
                    },

                    GUIAffect::CreateProjectRequested => {
                        gui.on_command(GUICommand::ShowCreateProjectWindow); 
                        Ok(None)
                    },

                    GUIAffect::CreateProjectInfo { 
                        project_name, 
                        project_path, 
                    } => {
                        Ok(
                            Some(
                                CreatingProjectEvent::CheckProjectInfo { 
                                    project_name: project_name, 
                                    project_path: project_path,
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
                            confirmation_context_manager
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
