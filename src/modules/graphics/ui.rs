mod modal_windows;
mod panels;
pub mod ui_affect;
mod ui_affects_processing;
mod ui_state_processing;

use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics::{
            events::EventBuffers,
            graphics_data::{
                ui_data::UIData,
            },
            graphics_states::{
                ui_state::UIState,
            },
        },
    },
};
use self::{
    panels::{
        central_panel::{central_panel, CentralPanelContext},
        right_panel::{right_panel, RightPanelContext},
        top_panel::{top_panel, TopPanelContext},
    },
    ui_affects_processing::{ui_affects_processing, UIAffectsProcessingContext},
    ui_state_processing::{ui_state_processing, UIStateProcessingContext},
};


pub struct UIContext<'c> {
    pub egui_context: &'c EGUIContext,
    pub event_buffers: &'c mut EventBuffers,
    pub ui_state: &'c mut UIState,
    pub ui_data: &'c mut UIData,
}

pub fn ui(ui_context: UIContext) {  
    top_panel(
        TopPanelContext { 
            egui_context: ui_context.egui_context,
            ui_affects: &mut ui_context.event_buffers.ui_affects,
            top_panel_data: &mut ui_context.ui_data.panels_data.top_panel_data
        }
    );
    right_panel(
        RightPanelContext { 
            egui_context: ui_context.egui_context, 
            ui_affects: &mut ui_context.event_buffers.ui_affects, 
        }
    );
    central_panel(
        CentralPanelContext { 
            egui_context: ui_context.egui_context, 
            ui_affects: &mut ui_context.event_buffers.ui_affects, 
        }
    );
    ui_state_processing(
        UIStateProcessingContext { 
            ui_state: ui_context.ui_state,
            egui_context: ui_context.egui_context,
            ui_data: ui_context.ui_data,
            ui_affects: &mut ui_context.event_buffers.ui_affects
        }
    );
    ui_affects_processing(
        UIAffectsProcessingContext { 
            event_buffers: ui_context.event_buffers, 
            ui_state: ui_context.ui_state, 
            egui_context: ui_context.egui_context,
        }
    );
} 

