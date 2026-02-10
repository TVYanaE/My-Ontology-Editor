mod resumed_event_handle; 

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
    resumed_event_handle::{
        resumed_event_handle,
        ResumedEventContext,
    },
};

pub struct CustomEventContext<'c> {
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
}

pub fn handle_custom_event(
    event: CustomEvent,
    custom_event_context: CustomEventContext,
) -> Option<GraphicsCoreState> {
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
            ); 

            None
        },
    }; 
    

    new_state_opt
}
