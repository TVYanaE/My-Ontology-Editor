use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::ITCEvent
            },
            graphics_core::GraphicsCoreState,
        },
    },
};

pub fn itcevent_handle(event: ITCEvent) -> Option<GraphicsCoreState> {
    match event {
        ITCEvent::AppShutdownReq => {
            Some(GraphicsCoreState::Shutdown)
        }
    }
} 
