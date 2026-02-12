
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{CustomEvent},
                EventBuffers,
            },
            graphics_states::{
                ui_state::{UIState, ModalWindow},
            },
            ui::{
                ui_affect::UIAffect
            },
        },
    },
};

pub struct UIAffectsProcessingContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub ui_state: &'c mut UIState, 
}

pub fn ui_affects_processing(
    ui_affects_processing_context: UIAffectsProcessingContext,
) {
    while let Some(affect) = ui_affects_processing_context.event_buffers.ui_affects.pop_front() {
        match affect {
            UIAffect::QuitButtonPressed => {
                ui_affects_processing_context
                .event_buffers
                .custom_events
                .send_event(CustomEvent::AppShutdownReq)
                .expect("Event Loop was closed");
            },
            UIAffect::CreateNewProjectButtonPressed => {
                *ui_affects_processing_context.ui_state = UIState::ModalWindowOpen(
                    ModalWindow::CreateNewProjectWindow
                ); 
            },
            UIAffect::CloseCreateNewProjectWindowButtonPressed => {
                *ui_affects_processing_context.ui_state = UIState::Idle;
           },
        }
    }
}
