
use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::SidePanel;

use crate::modules::app::gui::gui_event::GUIEventBuffer;

pub struct LeftPanel {
    visibility: bool,
}

impl LeftPanel {
    pub(super) fn new() -> Self {
        Self { 
            visibility: true, 
        }
    }

    pub(super) fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        SidePanel::left("Left-Panel").show(context, |left_panel_ui|{

        });
    } 
}
