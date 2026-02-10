use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{
                    GraphicsEvent,
                },
                GraphicsEventBuffer,
            },
        },
    },
};

pub fn register_graphics_event<T: Into<GraphicsEvent>>(
    graphics_event_buffer: &mut GraphicsEventBuffer,
    event: T,
) {
    graphics_event_buffer.push_back(event.into());
}
