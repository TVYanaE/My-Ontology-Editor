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
                graphics_event::{ExternalEvent},
            },
            graphics_states::{
                ui_state::{
                    ui_general_state::UIGeneralState,
                    UIState,
                },
            },
            graphics_core::GraphicsCoreState,
        },
    },
};

pub struct ExternalEventContext<'c> {
    pub ui_state: &'c mut UIState,
}

#[instrument(skip_all,err)]
pub fn handle_external_event(
    event: ExternalEvent,
    itc_event_context: ExternalEventContext,
) -> Result<Option<GraphicsCoreState>, ExternalEventError> {
    match event {
        ExternalEvent::AppShutdownReq => {
            Ok(Some(GraphicsCoreState::Shutdown))
        }
        ExternalEvent::TaskDone => { 
            itc_event_context.ui_state.ui_general_state = UIGeneralState::Idle; 
            Ok(Some(GraphicsCoreState::Runnig))
        }
    }
}

#[derive(Debug, Error)]
pub enum ExternalEventError {

}
