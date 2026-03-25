
use eframe::egui::Context as EGUIContext;
use eframe::egui::Ui as EGUIUi;

use eframe::egui::containers::modal::Modal;
use eframe::egui::containers::Sides;

use eframe::egui::widgets::Button;

use crate::modules::app::gui::gui_event::{GUIEvent, GUIEventBuffer};

pub struct CreateSemanticNodeWindow {
    name: String,
}

impl CreateSemanticNodeWindow {
    pub fn new() -> Self {
        Self { 
            name: String::with_capacity(64), 
        }
    }
    pub fn prepare(
        &mut self,
        ctx: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        Modal::new("Create-Semantic-Node-Window".into()).show(ctx, |ui|{
            bottom_panel(ui, event_buffer);          
        });
    }
}

fn bottom_panel(
    ui: &mut EGUIUi,
    event_buffer: &mut GUIEventBuffer,
) {
    let (left_resp, right_resp) = Sides::new().show(ui, 
        |left_ui|{
            left_ui.add(Button::new("Create")) 
        }, 
        |right_ui|{
            right_ui.add(Button::new("Cancel"))
        },
    ); 

    if left_resp.clicked() { 
    };

    if right_resp.clicked() {
        event_buffer.push(GUIEvent::CreateSemanticNodeWindowClosed);
    };
}

