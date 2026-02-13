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
                graphics_event::{CustomEvent, ITCEvent},
            },
            graphics_data::GraphicsData,
            graphics_states::{
                GraphicsStates,
                ui_state::{
                    ui_general_state::UIGeneralState,
                    UIState,
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
            custom_event_context
                .logic_core
                .as_ref()
                .ok_or_else(||{
                    CustomEventError::LogicThreadWasntFound
                })?
                .logic_event_channel_sender
                .send(LogicEvent::CreateProject(
                    ProjectDescriptor { 
                        project_name: req.project_name, 
                        project_dir: req.project_dir 
                    }
                ))?;

            custom_event_context.graphics_states.ui_state.ui_general_state = UIGeneralState::WaitingBlocingTask; 
            Some(GraphicsCoreState::Waiting)
        },
        CustomEvent::ITCEvent(event) => {
            itc_event_handle(
                event,
                ITCEventContext { 
                    ui_state: &mut custom_event_context.graphics_states.ui_state
                }
            )? 
        },
    }; 
    Ok(new_state_opt)
}

pub struct ITCEventContext<'c> {
    pub ui_state: &'c mut UIState,
}

pub fn itc_event_handle(
    event: ITCEvent,
    itc_event_context: ITCEventContext,
) -> Result<Option<GraphicsCoreState>, CustomEventError> {
    match event {
        ITCEvent::AppShutdownReq => {
            Ok(Some(GraphicsCoreState::Shutdown))
        }
        ITCEvent::TaskDone => { 
            itc_event_context.ui_state.ui_general_state = UIGeneralState::Idle; 
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

    #[error("Critical Error. Logic Thread wasn't found")]
    LogicThreadWasntFound,

    #[error("Send error flume: {0}")]
    SendError(#[from] flume::SendError<LogicEvent>),
}

