
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::modal::Modal;
use eframe::egui::widgets::{Label, Button};

use super::super::gui_event::{GUIEvent, GUIEventBuffer};

pub struct Notification {
}

impl Notification {
    pub(super) fn new() -> Self {
        Self { 
        }
    }
    pub(super) fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
        text: &str,
    ) {
        Modal::new("Notification".into())
            .show(context, |notification_ui|{
                notification_ui.add(Label::new(text));

                if notification_ui.add(Button::new("Ok")).clicked() {
                    event_buffer.push(GUIEvent::NotificationClosed);
                }
            }
        );
    } 
}
