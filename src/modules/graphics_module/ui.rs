mod modal_window;
mod panels;
mod ui_event;
mod ui_logic;

use crate::{
    aliases::{
        EGUIContext,
    }, 
};
use self::{
    modal_window::ModalWindow,
    panels::Panels,
    ui_event::{UIEvent, UIEvents},
    ui_logic::UILogic,
};
pub use self::{
    ui_event::CreateProjectRequest
};

#[derive(Debug)]
pub enum UIAffect {
    ExitRequested,
    CreateProjectReq(CreateProjectRequest),
}

#[derive(Debug)]
pub enum UIInputEvent {
    Waiting,
    StopWaiting,
}

#[derive(Default)]
pub struct UI {
    ui_events: UIEvents,
    panels: Panels,
    modal_window: ModalWindow,
}

impl UI {
    pub fn prepare_ui(
        &mut self,
        egui_context: &EGUIContext,               
    ) -> Vec<UIAffect> {
        let mut panels_events = self.panels.prepare(egui_context);
        let modal_window_events = self.modal_window.prepare(egui_context);
       
        panels_events.extend(modal_window_events); 

        let ui_affects = UILogic::ui_events_processing(
            panels_events,
            &mut self.panels, 
            &mut self.modal_window
        );

        ui_affects
    }
    pub fn on_event(&mut self, event: UIInputEvent) {
        UILogic::ui_input_event_processing(event, &mut self.modal_window);
    }
}
 

