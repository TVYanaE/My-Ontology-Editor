mod top_panel;

use eframe::egui::Context as EGUIContext;

use super::gui_event::GUIEventBuffer;

use self::top_panel::TopPanel;

pub struct MainGUI {
    top_panel: TopPanel,
}

impl MainGUI {
    pub fn new() -> Self {
        Self {  
            top_panel: TopPanel::new(),
        }
    }
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        self.top_panel.prepare(context, event_buffer); 
    }
}
