
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{CustomEvent, GraphicsEvent},
                EventBuffers,
            },
            ui::{
                ui_affect::UIAffect
            },
        },
    },
};


pub fn ui_affects_processing(
    event_buffers: &mut EventBuffers
) {
    while let Some(affect) = event_buffers.ui_affects.pop_front() {
        match affect {
            UIAffect::QuitButtonPushed => {
                event_buffers.graphics_event_buffer.push_back(GraphicsEvent::CustomEvent(
                    CustomEvent::AppShutdownReq
                ));
            },
        }
    }
}
