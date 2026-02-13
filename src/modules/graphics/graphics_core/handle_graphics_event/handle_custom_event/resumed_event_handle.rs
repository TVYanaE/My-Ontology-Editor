mod init_egui;
mod init_wgpu;

use winit::{
    window::Window,
};
use crate::{
    modules::{
        graphics::{
            graphics_data::GraphicsData,
            graphics_states::{
                GraphicsStates,
            },
        },
    },
};
use super::{
    CustomEventError,
};
use self::{
    init_egui::{
        init_egui, InitEGUIContext,
    },
    init_wgpu::{
        init_wgpu, InitWGPUContext
    },
};

pub struct ResumedEventContext<'c> {
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData, 
}

pub fn resumed_event_handle(
    window: Window,
    resumed_event_context: ResumedEventContext,
) -> Result<(), CustomEventError> {
    init_wgpu(
        InitWGPUContext { 
            wgpu_state: &mut resumed_event_context
                .graphics_states
                .graphics_backend_state
                .wgpu_state, 
            wgpu_data: &mut resumed_event_context
                .graphics_data
                .graphics_backend_data
                .wgpu_data, 
            window 
        }
    )?;
    init_egui(
        InitEGUIContext { 
            egui_state: &mut resumed_event_context
                .graphics_states
                .graphics_backend_state
                .egui_state, 
            wgpu_data: &resumed_event_context
                .graphics_data
                .graphics_backend_data
                .wgpu_data, 
            egui_data: &mut resumed_event_context
                .graphics_data
                .graphics_backend_data
                .egui_data
        }
    );
    Ok(())
}
