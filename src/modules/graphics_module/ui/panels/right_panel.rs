use egui::{
    containers::{
        panel::{
            SidePanel, Side, 
        },
    },
};
use crate::{
    aliases::{
        EGUIContext,
    }, 
};
use super::{
    UIEvent,
};

#[derive(Default)]
pub struct RightPanel;

impl RightPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Vec<UIEvent> {
        let ui_events = Vec::with_capacity(4);
        
        SidePanel::new(Side::Right, "Right-Panel")
            .resizable(false) 
            .show(egui_context, |_right_panel_ui|{

            });

        ui_events
    }
} 

