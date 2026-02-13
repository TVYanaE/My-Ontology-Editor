mod resumed_event_handle; 

use thiserror::{
    Error,
};
use tracing::{
    instrument
};
use crate::{
    modules::{
        logic::{
            events::LogicEvent,
            logic_core::LogicCore
        },
        graphics::{
            events::{
                graphics_event::{CustomEvent, ITCEvent},
            },
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
            graphics_core::GraphicsCoreState,
        },
    },
};
use self::{
    resumed_event_handle::{
        resumed_event_handle,
        ResumedEventContext,
    },
};

pub struct CustomEventContext<'c> {
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
    pub logic_core: &'c mut Option<LogicCore>,
}

#[instrument(skip_all, err)]
pub fn handle_custom_event(
    event: CustomEvent,
    custom_event_context: CustomEventContext,
) -> Result<Option<GraphicsCoreState>, CustomEventError> {
    let new_state_opt = match event {
        CustomEvent::AppShutdownReq => {
            // TODO Logic For Graceful shutdown
            if let Some(logic_core) = custom_event_context.logic_core.take() {
                if let Err(_) = logic_core.logic_event_channel_sender.send(LogicEvent::Shutdown) {
                    return Ok(Some(GraphicsCoreState::Shutdown));
                }
                logic_core.handle.join().unwrap();
                Some(GraphicsCoreState::Shutdown)
            }
            else {
                Some(GraphicsCoreState::Shutdown)
            } 
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
        CustomEvent::CreateProjectReq(req) => {
             
            Some(GraphicsCoreState::Waiting)
        },
        CustomEvent::ITCEvent(event) => {
            itc_event_handle(event)? 
        },
    }; 
    Ok(new_state_opt)
}

pub fn itc_event_handle(
    event: ITCEvent
) -> Result<Option<GraphicsCoreState>, CustomEventError> {
    match event {
        ITCEvent::AppShutdownReq => {
            Ok(Some(GraphicsCoreState::Shutdown))
        }
        ITCEvent::ResponseDone => {
            Ok(Some(GraphicsCoreState::Runnig))
        }
    }
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

