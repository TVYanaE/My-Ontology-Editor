
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::{TopBottomPanel, TopBottomSide};

use crate::modules::app::gui::gui_event::GUIEventBuffer;

pub struct BottomPanel {

}

impl BottomPanel {
    pub fn new() -> Self {
        Self {  
        }
    }
    pub fn prepare(
        &mut self,
        ctx: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        TopBottomPanel::new(TopBottomSide::Bottom, "Bottom-Panel")
            .show(ctx, |bottom_panel_ui|{

            });
    } 
}
