use egui::{
    containers::{
        panel::{
            CentralPanel,
        },
    },
};
use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics::{
            events::UIAffects
        },
    },
};

pub struct CentralPanelContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub ui_affects: &'c mut UIAffects,
}

pub fn central_panel(
    central_panel_context: CentralPanelContext,
) {
    CentralPanel::default().show(central_panel_context.egui_context, |central_panel_ui|{
         
    });
}
