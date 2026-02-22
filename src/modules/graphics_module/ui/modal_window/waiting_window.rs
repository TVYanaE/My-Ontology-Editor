use egui::{
    containers::{
        Modal,
    },
    widgets::{
        Spinner,
        Label,
    },
};
use crate::{
    aliases::{EGUIContext},
}; 

pub struct WaitingWindow{
    text: String
}

impl Default for WaitingWindow {
    fn default() -> Self {
        Self { 
            text: String::with_capacity(64) 
        }
    } 
}

impl WaitingWindow {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) {
        Modal::new("Waiting-Spinner".into()).show(egui_context, |ui|{
            ui.add(Label::new(&self.text));
            ui.add(Spinner::new());
        });  
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.clear();
        self.text.push_str(text);
    }
}
