use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::Modal;
use eframe::egui::widgets::{Label, Button};

use super::super::gui_command::ConfirmationType;
use super::super::gui_event::{GUIEvent, GUIEventBuffer};

pub struct ConfirmationWindow {
}

impl ConfirmationWindow {
    pub fn new() -> Self {
        Self {  
        }
    } 
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        confirmation_text: &str,
        confirmation_type: &ConfirmationType,
    ) {
        Modal::new("Notification".into())
            .show(context, |confirmation_window_ui|{
                confirmation_window_ui.add(Label::new(confirmation_text));

                if confirmation_window_ui.add(Button::new("Yes")).clicked() {
                    event_buffer.push(
                        GUIEvent::ConfirmationObtain { 
                            confirmation_type: confirmation_type.clone(), 
                            decision: true, 
                        }
                    );
                }

                if confirmation_window_ui.add(Button::new("No")).clicked() {
                    event_buffer.push(
                        GUIEvent::ConfirmationObtain { 
                            confirmation_type: confirmation_type.clone(), 
                            decision: false, 
                        }
                    );
                }
            }
        ); 
    }
}
