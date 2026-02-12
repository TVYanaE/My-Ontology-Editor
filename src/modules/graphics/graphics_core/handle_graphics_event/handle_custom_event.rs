mod itc_event_handle;
mod resumed_event_handle; 

use thiserror::{
    Error,
};
use tracing::{
    instrument
};
use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::CustomEvent,
            },
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
            graphics_core::GraphicsCoreState,
        },
    },
};
use self::{
    itc_event_handle::itcevent_handle,
    resumed_event_handle::{
        resumed_event_handle,
        ResumedEventContext,
    },
};

pub struct CustomEventContext<'c> {
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
}

#[instrument(skip_all, err)]
pub fn handle_custom_event(
    event: CustomEvent,
    custom_event_context: CustomEventContext,
) -> Result<Option<GraphicsCoreState>, CustomEventError> {
    let new_state_opt = match event {
        CustomEvent::AppShutdownReq => {
            // TODO Logic For Graceful shutdown
            Some(GraphicsCoreState::Shutdown)
        }, 
        CustomEvent::ResumedEvent(window) => {
            resumed_event_handle(
                window, 
                ResumedEventContext { 
                    graphics_states: custom_event_context.graphics_states, 
                    graphics_data: custom_event_context.graphics_data 
                }
            )?;

            None
        },
        CustomEvent::ITCEvent(event) => {
            itcevent_handle(event)
        },
    }; 
    Ok(new_state_opt)
}

#[derive(Debug, Error)]
pub enum CustomEventError {
    #[error("Request Device Error: {0} ")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    #[error("Request Adapter Error: {0}")]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),

    #[error("Create Surface Error: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    #[error("Choosed Texture Format isn't supported")]
    TextureFormatIsntSupported,
}
