
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{CustomEvent},
                EventBuffers,
            },
            ui::{
                ui_affect::UIAffect
            },
        },
    },
};


pub fn ui_affects_processing(
    event_buffers: &mut EventBuffers,
) {
    while let Some(affect) = event_buffers.ui_affects.pop_front() {
        match affect {
            UIAffect::QuitButtonPushed => {
                event_buffers.custom_events.send_event(CustomEvent::AppShutdownReq).expect("Event Loop was closed");
            },
        }
    }
}
