use egui::{
    containers::{
        Modal, Sides,
    },
    widgets::{
        Button, Label,
    }, 
};
use crate::{
    aliases::{EGUIContext},
    modules::{
        graphics_module::{
            ui::{
                events::{UIEvents, UIEvent},
                ui_error::UIError,
            },
        },
        logic_module::{
            events::{ConfirmationID, ConfirmationKind}
        },
    },
}; 

pub struct ConfirmationWindow {
    text: String,
    confirmation_id: Option<ConfirmationID>,
    confirmation_kind: Option<ConfirmationKind>,
}

impl Default for ConfirmationWindow {
    fn default() -> Self {
        Self { 
            text: String::with_capacity(64), 
            confirmation_id: None,
            confirmation_kind: None,
        }
    }
}

impl ConfirmationWindow {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(2);

        Modal::new("Confirmation-Window".into()).show(egui_context, |ui|{
            ui.add(Label::new(&self.text));
            let (left_resp, right_resp) = Sides::new().show(ui, 
                |left_ui|{
                    left_ui.add(Button::new("Yes")) 
                }, 
                |right_ui|{
                    right_ui.add(Button::new("No"))
                }
            ); 

            if left_resp.clicked() {
                ui_events.push(UIEvent::ConfirmationDecision { 
                    confirmation_id: self.confirmation_id.take().unwrap(), 
                    decision: true, 
                    decision_kind: self.confirmation_kind.take().unwrap().into(),
                });
            }
            if right_resp.clicked() {       
                ui_events.push(UIEvent::ConfirmationDecision { 
                    confirmation_id: self.confirmation_id.take().unwrap(), 
                    decision: false, 
                    decision_kind: self.confirmation_kind.take().unwrap().into(),
                });
            }
        }); 

        Ok(ui_events)
    }
    
    pub fn set_confirmation(
        &mut self, 
        confirmation_id: ConfirmationID, 
        text: &str,
        confirmation_kind: ConfirmationKind,
    ) {
        self.confirmation_id = Some(confirmation_id);
        self.confirmation_kind = Some(confirmation_kind);
        self.text.clear(); 
        self.text.push_str(text);
    }
}
