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
    modules::{
        graphics_module::{
            ui::{
                events::UIEvents,
                ui_error::UIError,
            },
        },
    },
};


#[derive(Default)]
pub struct RightPanel;

impl RightPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Result<UIEvents, UIError> {
        let ui_events = Vec::with_capacity(4);
        
        SidePanel::new(Side::Right, "Right-Panel")
            .resizable(false) 
            .show(egui_context, |_right_panel_ui|{

            });

        Ok(ui_events)
    }
} 

