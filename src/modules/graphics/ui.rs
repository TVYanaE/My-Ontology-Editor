mod panels;
pub mod ui_affect;
mod ui_affects_processing;

use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics::{
            events::EventBuffers,
        },
    },
};
use self::{
    panels::{
        top_panel::top_panel
    },
    ui_affects_processing::ui_affects_processing,
};


pub struct UIContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub event_buffers: &'c mut EventBuffers,
}

pub fn ui(mut ui_context: UIContext) {
    top_panel(
        ui_context.egui_context,
        &mut ui_context.event_buffers.ui_affects
    );
    ui_affects_processing(&mut ui_context.event_buffers);
} 


