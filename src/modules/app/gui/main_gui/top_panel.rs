mod main_bar;

use eframe::egui::Context as EGUIContext;
use eframe::egui::containers::panel::{
    TopBottomPanel, TopBottomSide,
};

use crate::modules::app::gui::gui_event::GUIEventBuffer;

use self::main_bar::MainBar;

pub struct TopPanel {
    main_bar: MainBar
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            main_bar: MainBar::new(),
        }
    }
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        event_buffer: &mut GUIEventBuffer,
    ) {
        TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
            .show(context, |top_panel_ui|{
                self.main_bar.prepare(top_panel_ui, event_buffer);
            }
        ); 
    }
}
