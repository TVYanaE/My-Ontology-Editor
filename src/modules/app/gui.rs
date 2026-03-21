pub mod gui_affect;
pub mod gui_command;
pub mod gui_error; 
mod gui_event; 
mod gui_event_handling;
mod gui_state; 
mod main_gui; 
mod modal_window; 

use eframe::egui::Context as EGUIContext;

use self::gui_affect::GUIAffectBuffer;
use self::gui_command::GUICommand;
use self::gui_error::GUIError;
use self::gui_event::GUIEventBuffer;
use self::gui_event_handling::gui_event_handling;
use self::gui_state::{
    GUIState, GUIStateTransform,
    ModalWindowType,
};
use self::main_gui::MainGUI;
use self::modal_window::ModalWindow;

pub struct GUI {
    prev_state: Option<GUIState>,
    current_state: GUIState, 
    event_buffer: GUIEventBuffer,
    main_gui: MainGUI,
    modal_window: ModalWindow,
}

impl GUI {
    pub fn new() -> Self {
        Self {
            prev_state: None,
            current_state: GUIState::default(),
            event_buffer: GUIEventBuffer::with_capacity(8),
            main_gui: MainGUI::new(),
            modal_window: ModalWindow::new(),
        }
    }

    pub fn prepare_gui(
        &mut self,
        context: &EGUIContext,
    ) -> Result<GUIAffectBuffer, GUIError>{
        let mut gui_affect_buffer = GUIAffectBuffer::with_capacity(4);
        
        self.main_gui.prepare(context, &mut self.event_buffer);

        match &self.current_state {
            GUIState::Idle => {},
            GUIState::ShowModalWindow(modal_window_type) => {
                self.modal_window.prepare(
                    context, 
                    modal_window_type, 
                    &mut self.event_buffer
                ); 
            },
        }

        match gui_event_handling(
            self.event_buffer.drain(), 
            &mut gui_affect_buffer,
            &mut self.modal_window,
        ) {
            GUIStateTransform::Stay => {},
            GUIStateTransform::Next(next_state) => {
                let prev_state = std::mem::replace(
                    &mut self.current_state,
                    next_state, 
                );

                self.prev_state = Some(prev_state);
            },
            GUIStateTransform::Prev => {
                if let Some(prev_state) = &self.prev_state {
                    let new_prev = std::mem::replace(
                        &mut self.current_state, 
                        prev_state.clone()
                    );

                    self.prev_state = Some(new_prev);
                }
                else {
                    let new_prev = std::mem::replace(
                        &mut self.current_state, 
                        GUIState::Idle
                    );

                    self.prev_state = Some(new_prev); 
                }
            },
        }; 

        Ok(gui_affect_buffer)
    }

    pub fn on_command(
        &mut self,
        command: GUICommand 
    ) {
        match command { 
            GUICommand::ShowNotification(text) => {
                let prev_state = std::mem::replace(
                    &mut self.current_state, 
                    GUIState::ShowModalWindow(ModalWindowType::Notification(text)), 
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
        }
    }
}
