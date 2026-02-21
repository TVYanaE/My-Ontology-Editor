use egui::{
    containers::{
        modal::{
            Modal,
        }, 
    },
    widgets::{ 
        Button, Label,
    },
};
use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics_module::{
            ui::{
                events::{UIEvent, UIEvents},
                ui_error::UIError,
            },
        },
    },
};


pub struct NotificationData {
    text: String,
}

impl Default for NotificationData {
    fn default() -> Self {
        Self { 
            text: String::with_capacity(64) 
        }
    }
}

#[derive(Default)]
pub struct Notification {
    data: NotificationData
}

impl Notification {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(4);
            prepare_notification(egui_context, &mut ui_events, &self.data.text);
        Ok(ui_events)
    }

    pub fn set_notification_text(&mut self, text: &str) {
        self.data.text.clear();
        self.data.text.push_str(text);
    }
}

fn prepare_notification(
    egui_context: &EGUIContext,
    ui_events: &mut Vec<UIEvent>,
    text: &str,
) {
    Modal::new("Create-New-Project-Window-Notification".into()).show(
        egui_context, |notificatio_ui| {
            notificatio_ui.add(Label::new(text));
            if notificatio_ui.add(Button::new("Ok")).clicked() {
                ui_events.push(UIEvent::NotificationClosed); 
            } 
        }
    ); 
}
