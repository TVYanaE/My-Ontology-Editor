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
        graphics::{
            events::{
                UIAffects,
            },
        },
    },
};

pub struct RightPanelContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub ui_affects: &'c mut UIAffects,
}

pub fn right_panel(
    right_panel_context: RightPanelContext,
) {
    SidePanel::new(Side::Right, "Right-Panel")
        .resizable(false) 
        .show(right_panel_context.egui_context, |right_panel_ui|{

        });
}
