use eframe::{
    egui,
};
use crate::{
    aliases::{
        EFrame, EFrameCreationContext,
        EGUIContext,
    },
};

#[derive(Default)]
pub struct Application {

}

impl Application {
    pub fn new(_creation_context: &EFrameCreationContext<'_>) -> Self {
        
        Self::default()
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &EGUIContext, _frame: &mut EFrame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello World!");
       }); 
    } 
}
