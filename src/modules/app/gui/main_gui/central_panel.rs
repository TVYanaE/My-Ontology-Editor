use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::CentralPanel as EGUICentralPanel;

use crate::modules::app::gui::gui_event::GUIEventBuffer;

pub struct CentralPanel {
}

impl CentralPanel {
    pub fn new() -> Self {
        Self { 
        }
    }

    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        EGUICentralPanel::default().show(context, |cental_panel_ui|{

        });
    } 
}
