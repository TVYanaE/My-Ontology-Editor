pub mod main_bar;

use egui::{
    containers::{
        panel::{
            TopBottomPanel, TopBottomSide
        },
    },
};
use crate::{
    aliases::{
        EGUIContext 
    }, 
};
use super::{
    UIEvent
};
use self::{
    main_bar::MainBar, 
};

#[derive(Default)]
pub struct TopPanel{
    main_bar: MainBar
}

impl TopPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Vec<UIEvent> {
        let mut ui_events = Vec::with_capacity(8);

        TopBottomPanel::new(TopBottomSide::Top, "Top-Panel")
        .show(egui_context, |top_panel_ui|{
            let main_bar_events = self.main_bar.prepare(top_panel_ui);
            ui_events.extend(main_bar_events.into_iter());
        });

        ui_events
    }
}
