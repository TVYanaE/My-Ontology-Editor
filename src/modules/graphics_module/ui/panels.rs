mod central_panel;
mod right_panel;
mod top_panel;

use crate::{
    aliases::{
        EGUIContext
    },
};
use super::{
    UIEvent,
};
use self::{
    central_panel::CentralPanel,
    right_panel::RightPanel,
    top_panel::TopPanel,
};

#[derive(Default)]
pub struct Panels {
    pub central_panel: CentralPanel,
    pub right_panel: RightPanel,
    pub top_panel: TopPanel
}

impl Panels {
    pub fn prepare(&mut self, egui_context: &EGUIContext) -> Vec<UIEvent> {
        let mut ui_events: Vec<UIEvent> = Vec::with_capacity(8);

        let top_panel_events = self.top_panel.prepare(egui_context);
        let right_panel_events = self.right_panel.prepare(egui_context);
        let central_panel_events = self.central_panel.prepare(egui_context);

        let all_events = top_panel_events
            .into_iter()
            .chain(right_panel_events)
            .chain(central_panel_events);

        ui_events.extend(all_events);

        ui_events
    }
}
