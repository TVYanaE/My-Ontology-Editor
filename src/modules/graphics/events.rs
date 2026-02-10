pub mod graphics_event;

use std::{
    collections::VecDeque
};
use crate::{
    modules::{
        graphics::{
            ui::ui_affect::UIAffect,
        },
    },
};
use self::{
    graphics_event::GraphicsEvent,
};

pub type GraphicsEventBuffer = VecDeque<GraphicsEvent>;
pub type UIAffects = VecDeque<UIAffect>;

pub struct EventBuffers {
    pub graphics_event_buffer: GraphicsEventBuffer,
    pub ui_affects: UIAffects,
}

impl Default for EventBuffers {
    fn default() -> Self {
        Self { 
            graphics_event_buffer: GraphicsEventBuffer::with_capacity(16), 
            ui_affects: UIAffects::with_capacity(16),
        }
    }
}
