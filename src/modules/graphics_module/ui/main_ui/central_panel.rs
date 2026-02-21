use crate::{
    aliases::{
        EGUIContext, EGUICentralPanel
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
pub struct CentralPanel;

impl CentralPanel {
    pub(super) fn prepare(
        &mut self,
        egui_context: &EGUIContext
    ) -> Result<UIEvents, UIError> {
        let ui_events = Vec::with_capacity(4);
        EGUICentralPanel::default().show(egui_context, |_central_panel_ui|{
         
        });
        
        Ok(ui_events)
    } 
}


