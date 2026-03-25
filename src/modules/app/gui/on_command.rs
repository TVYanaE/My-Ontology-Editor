
use crate::modules::app::gui::GUI;
use crate::modules::app::gui::gui_state::{
    GUIState, ModalWindowType,
};
use crate::modules::app::gui::gui_command::GUICommand;

impl GUI {
    pub fn on_command(
        &mut self,
        command: GUICommand 
    ) {
        match command { 
            GUICommand::ShowNotification(text) => {
                let prev_state = std::mem::replace(
                    &mut self.current_state, 
                    GUIState::ShowModalWindow(
                        ModalWindowType::Notification(text)
                    ), 
                );

                self.prev_state = Some(prev_state); 
            },

            GUICommand::ShowConfirmationWindow { 
                confirmation_type, 
                confirmation_text, 
            } => {
                let prev_state = std::mem::replace(
                    &mut self.current_state, 
                    GUIState::ShowModalWindow(
                        ModalWindowType::ConfirmationWindow { 
                            confirmation_text, 
                            confirmation_type,
                        }
                    ), 
                );

                self.prev_state = Some(prev_state);
            },

            GUICommand::ShowLoading => {
                let prev_state = std::mem::replace(
                    &mut self.current_state, 
                    GUIState::ShowModalWindow(
                        ModalWindowType::LoadingWindow
                    ), 
                );

                self.prev_state = Some(prev_state);
            },

            GUICommand::StopShowLoading => {
                if let Some(prev_state) = &self.prev_state {
                    let new_prev_state = std::mem::replace(
                        &mut self.current_state, 
                        prev_state.clone(), 
                    );

                    self.prev_state = Some(new_prev_state);      
                } else {
                    let prev_state = std::mem::replace(
                        &mut self.current_state, 
                        GUIState::Idle, 
                    );

                    self.prev_state = Some(prev_state);    
                }
            },

            GUICommand::ShowMainUI => {
                let prev_state = std::mem::replace(
                    &mut self.current_state, 
                    GUIState::Idle, 
                );

                self.prev_state = Some(prev_state); 
            }, 

            GUICommand::ProjectOpened { 
                project_id 
            } => {
                self.modal_window.with_open_project_window(|open_project_window|{
                    open_project_window.clear();
                });

                self.selected_project = Some(project_id);
            },

            GUICommand::ProjectCreated { 
                project_id 
            } => {
                self.modal_window.with_create_project_window(|create_project_window|{
                    create_project_window.clear();
                });

                self.selected_project = Some(project_id);
            },

            GUICommand::SelectProject { 
                project_id 
            } => {
                self.selected_project = Some(project_id);
            },
        }
    }
}


