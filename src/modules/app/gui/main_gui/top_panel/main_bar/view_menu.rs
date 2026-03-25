
use eframe::egui::Ui as EGUIUi;

use crate::modules::app::gui::gui_event::{GUIEventBuffer, GUIEvent};

pub struct ViewMenu {
    left_panel_visibility: bool,
}

impl ViewMenu {
    pub fn new() -> Self {
        Self {  
            left_panel_visibility: true,
        }
    }
    
    pub fn prepare(
        &mut self, 
        ui: &mut EGUIUi,
        event_buffer: &mut GUIEventBuffer,
    ) {
        if ui.checkbox(&mut self.left_panel_visibility, "Left Panel").clicked() {
            event_buffer.push(
                GUIEvent::SetLeftPanelVisibility { 
                    visibility: self.left_panel_visibility 
                }
            ); 
        }
    }
}
