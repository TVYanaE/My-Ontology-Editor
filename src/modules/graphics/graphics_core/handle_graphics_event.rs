mod handle_external_event;
mod handle_grahic_event_error;
mod handle_internal_event;
mod handle_window_event;

use winit::{
    event::WindowEvent,
};
use crate::{
    modules::{ 
        graphics::{
            events::{
                graphics_event::{
                    GraphicsEvent, CustomEvent,
                },
                EventBuffers,
            },
            graphics_core::{
                GraphicsCoreState,
                LogicThreadDescriptor,
            },
            graphics_data::GraphicsData,
            graphics_states::GraphicsStates,
        },
    },
};
use self::{
    handle_external_event::{
        handle_external_event, 
        ExternalEventContext,
        ExternalEventError
    }, 
    handle_grahic_event_error::{
        handle_graphic_event_error
    }, 
    handle_internal_event::{
        handle_internal_event, 
        InternalEventContext,
        InternalEventError
    },
    handle_window_event::{
        handle_window_event,
        HandleWindowEventContext,
        WindowEventError
    }
};


pub struct GraphicsApplicationContext<'c> {
    pub event_buffers: &'c mut EventBuffers,
    pub graphics_core_state: &'c mut GraphicsCoreState,
    pub graphics_states: &'c mut GraphicsStates,
    pub graphics_data: &'c mut GraphicsData,
    pub logic_thread_descriptor: &'c mut LogicThreadDescriptor,
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
                    match event {
                        CustomEvent::InternalEvent(event) => {
                            match handle_internal_event(
                                event,
                                InternalEventContext {
                                    graphics_data: graphics_application_context.graphics_data,
                                    graphics_states: graphics_application_context.graphics_states,
                                    logic_thread_descriptor: graphics_application_context.logic_thread_descriptor,
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
                        CustomEvent::ExternalEvent(event) => {
                            match handle_external_event(
                                event,
                                ExternalEventContext {
                                    ui_state: &mut graphics_application_context.graphics_states.ui_state
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
                        }
                    } 
                }, 
            } 
        },
        (GraphicsCoreState::Waiting, event) => {
            match event {
                GraphicsEvent::CustomEvent(event) => {
                    match event {
                        CustomEvent::ExternalEvent(event) => {
                            match handle_external_event(
                                event,
                                ExternalEventContext { 
                                    ui_state: &mut graphics_application_context.graphics_states.ui_state 
                                },
                            ) {
                                Ok(Some(new_state)) => new_state,
                                Ok(None) => GraphicsCoreState::Waiting,
                                Err(error) => {
                                    handle_graphic_event_error(
                                        error.into(),
                                        &graphics_application_context.event_buffers.custom_events
                                    );
                                    GraphicsCoreState::Runnig 
                                },
                            } 
                        },
                        _ => {GraphicsCoreState::Waiting}
                    }
                },
                GraphicsEvent::WindowEvent(event) => {
                    match event {
                        WindowEvent::CloseRequested |
                        WindowEvent::Resized(_)
                        => {
                            match handle_window_event(
                                event, 
                                HandleWindowEventContext { 
                                    event_buffers: graphics_application_context.event_buffers, 
                                    graphics_states: graphics_application_context.graphics_states, 
                                    graphics_data: graphics_application_context.graphics_data 
                                }
                            ) {
                                Ok(Some(new_state)) => new_state,
                                Ok(None) => GraphicsCoreState::Waiting,
                                Err(error) => {
                                    handle_graphic_event_error(
                                        error.into(),
                                        &graphics_application_context.event_buffers.custom_events
                                    );
                                    GraphicsCoreState::Runnig
                                },
                            }
                        }, 
                        _ => {
                            GraphicsCoreState::Waiting
                        }
                    }
                },
            }
        },
        (current_state, _) => current_state.clone(), 
    };
}

pub enum GraphicsEventError {
    ExternalEventError(ExternalEventError),
    WindowEventError(WindowEventError),
    InternalEventError(InternalEventError)
}
