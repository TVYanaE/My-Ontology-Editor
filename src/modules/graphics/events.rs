pub mod graphics_event;

use std::{
    collections::VecDeque
};
use winit::{
    event_loop::EventLoopProxy,
};
use crate::{
    modules::{
        graphics::{
            ui::ui_affect::UIAffect,
        },
    },
};
use self::{
    graphics_event::CustomEvent
};

pub type CustomEvents = EventLoopProxy<CustomEvent>;
pub type UIAffects = VecDeque<UIAffect>;

pub struct EventBuffers {
    pub custom_events: CustomEvents,
    pub ui_affects: UIAffects,
}

impl EventBuffers {
    pub fn new(custom_events: CustomEvents) -> Self {
        Self {
            custom_events: custom_events,
            ui_affects: UIAffects::with_capacity(16),
        }
    }
}
