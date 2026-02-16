use egui::{
    containers::{
        Modal,
    },
    widgets::{
        Spinner,
    },
};
use crate::{
    aliases::{EGUIContext},
}; 

#[derive(Default)]
pub struct WaitingWindow;

impl WaitingWindow {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) {
        Modal::new("Waiting-Spinner".into()).show(egui_context, |ui|{
            ui.add(Spinner::new());
        });  
    }
}
