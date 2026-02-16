use crate::{
    aliases::{
        EGUIContext, EGUICentralPanel
    }, 
};
use super::{
    UIEvent,
};

#[derive(Default)]
pub struct CentralPanel;

impl CentralPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext
    ) -> Vec<UIEvent> {
        let ui_events = Vec::with_capacity(4);
        EGUICentralPanel::default().show(egui_context, |_central_panel_ui|{
         
        });
        
        ui_events
    } 
}


