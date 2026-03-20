
use eframe::egui::Ui as EGUIUI;
use eframe::egui::widgets::Button;

use super::super::super::super::gui_event::{GUIEventBuffer, GUIEvent};

pub struct FileMenu {

}

impl FileMenu {
    pub fn new() -> Self {
        Self {  
        }
    }
    pub fn prepare(
        &mut self,
        ui: &mut EGUIUI,
        event_buffer: &mut GUIEventBuffer,
    ) {
        if ui.add(Button::new("Create project")).clicked() {
            event_buffer.push(GUIEvent::CreateProjectButtonPressed); 
        };

        if ui.add(Button::new("Open project")).clicked() {
            event_buffer.push(GUIEvent::OpenProjectButtonPressed); 
        };

        if ui.add(Button::new("Exit")).clicked() {
            event_buffer.push(GUIEvent::ExitButtomPressed);
        }; 
    }
}
