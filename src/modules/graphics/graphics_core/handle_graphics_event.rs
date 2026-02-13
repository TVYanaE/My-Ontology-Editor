mod handle_custom_event;
mod handle_grahic_event_error;
mod handle_window_event;


use crate::{
    modules::{
        graphics::{
            events::{
                graphics_event::{
                    GraphicsEvent,
                },
                EventBuffers,
            },
            graphics_core::GraphicsCoreState,
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
        },
    },
};
use self::{
    handle_custom_event::{
        handle_custom_event,
        CustomEventContext,
    },
    handle_grahic_event_error::{
        handle_graphic_event_error
    }, 
    handle_window_event::{
        handle_window_event,
        HandleWindowEventContext,
    }
};


pub struct GraphicsApplicationContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub graphics_core_state: &'c mut GraphicsCoreState,
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
}

pub fn handle_graphics_event(
    graphics_application_context: GraphicsApplicationContext, 
    event: GraphicsEvent
) {
    let current_graphics_core_state = std::mem::replace(
        graphics_application_context.graphics_core_state,
        GraphicsCoreState::Processing,
    );
   
    *graphics_application_context.graphics_core_state = match (&current_graphics_core_state, event) {
        (GraphicsCoreState::Runnig, event) => {
            match event {
                GraphicsEvent::WindowEvent(event) => {
                    match handle_window_event(
                        event,
                        HandleWindowEventContext { 
                            event_buffers: graphics_application_context.event_buffers, 
                            graphics_states: graphics_application_context.graphics_states, 
                            graphics_data: graphics_application_context.graphics_data, 
                        }
                    ) {
                        Ok(Some(new_state)) => new_state,
                        Ok(None) => GraphicsCoreState::Runnig,
                        Err(error) => {
                            handle_graphic_event_error(
                                error.into(), 
                                &graphics_application_context.event_buffers.custom_events
                            );
                            GraphicsCoreState::Runnig
                        },
                    }     
                },
                GraphicsEvent::CustomEvent(event) => {
                    match handle_custom_event(
                        event,
                        CustomEventContext {
                            graphics_data: graphics_application_context.graphics_data,
                            graphics_states: graphics_application_context.graphics_states,
                        },
                    ) {
                        Ok(Some(new_state)) => new_state,
                        Ok(None) => GraphicsCoreState::Runnig,
                        Err(error) => {
                            handle_graphic_event_error(
                                error.into(),
                                &graphics_application_context.event_buffers.custom_events
                            );
                            GraphicsCoreState::Runnig 
                        },
                    } 
                }, 
            } 
        }, 
        (current_state, _) => current_state.clone(), 
    };
}
