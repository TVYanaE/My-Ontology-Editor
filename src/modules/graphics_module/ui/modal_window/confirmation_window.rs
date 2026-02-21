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
        shared::{
            task_id::TaskID,
        },
    },
}; 

pub struct ConfirmationWindow {
    text: String,
    task_id: Option<TaskID>,
}

impl Default for ConfirmationWindow {
    fn default() -> Self {
        Self { 
            text: String::with_capacity(64), 
            task_id: None, 
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
                ui_events.push(UIEvent::Confirmation { task_id: self.task_id.take().unwrap(), confirm: true });
            }
            if right_resp.clicked() {       
                ui_events.push(UIEvent::Confirmation { task_id: self.task_id.take().unwrap(), confirm: false });
            }
        }); 

        Ok(ui_events)
    }
    
    pub fn set_task(&mut self, task_id: TaskID, text: &str) {
        self.task_id = Some(task_id);
        self.text.clear(); 
        self.text.push_str(text);
    }
}
