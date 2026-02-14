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
            events::{LogicEvent, ProjectDescriptor},
            logic_core::LogicCore
        },
        graphics::{
            events::{
                graphics_event::{InternalEvent},
            },
            graphics_data::GraphicsData,
            graphics_states::{
                GraphicsStates,
                ui_state::{
                    ui_general_state::UIGeneralState,
                },
            },
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

pub struct InternalEventContext<'c> {
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
    pub logic_core: &'c mut Option<LogicCore>,
}

#[instrument(skip_all, err)]
pub fn handle_internal_event(
    event: InternalEvent,
    internal_event_context: InternalEventContext,
) -> Result<Option<GraphicsCoreState>, InternalEventError> {
    let new_state_opt = match event {
        InternalEvent::AppShutdownReq => {
            // TODO Logic For Graceful shutdown
            if let Some(logic_core) = internal_event_context.logic_core.take() {
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
        InternalEvent::ResumedEvent(window) => {
            resumed_event_handle(
                window, 
                ResumedEventContext { 
                    graphics_states: internal_event_context.graphics_states, 
                    graphics_data: internal_event_context.graphics_data 
                }
            )?;

            None
        },
        InternalEvent::CreateProjectReq(req) => {
            internal_event_context
                .logic_core
                .as_ref()
                .ok_or_else(||{
                    InternalEventError::LogicThreadWasntFound
                })?
                .logic_event_channel_sender
                .send(LogicEvent::CreateProject(
                    ProjectDescriptor { 
                        project_name: req.project_name, 
                        project_dir: req.project_dir 
                    }
                ))?;

            internal_event_context.graphics_states.ui_state.ui_general_state = UIGeneralState::WaitingBlocingTask; 
            Some(GraphicsCoreState::Waiting)
        }, 
    }; 
    Ok(new_state_opt)
}



#[derive(Debug, Error)]
pub enum InternalEventError {
    #[error("Request Device Error: {0} ")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    #[error("Request Adapter Error: {0}")]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),

    #[error("Create Surface Error: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    #[error("Choosed Texture Format isn't supported")]
    TextureFormatIsntSupported,

    #[error("Critical Error. Logic Thread wasn't found")]
    LogicThreadWasntFound,

    #[error("Send error flume: {0}")]
    SendError(#[from] flume::SendError<LogicEvent>),
}

