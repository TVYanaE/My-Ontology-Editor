mod graphics_core_logic;
pub mod graphics_event;
mod handle_graphics_event_error;


use winit::{
    event::WindowEvent,
};
use crate::{
    modules::{ 
        shared::{
            logic_module_handler::LogicModuleHandler,
        },
    },
};
use super::{ 
    CustomEvents,
    graphics_backend::{ 
        GraphicsBackend,
    },
    ui::UI,
};
use self::{
    graphics_core_logic::{
        GraphicsCoreLogic,
        InternalEventError, ExternalEventError, 
        WindowEventError, RedrawEventError,
    },
    graphics_event::{GraphicsEvent, CustomEvent, InternalEvent}, 
    handle_graphics_event_error::handle_graphic_event_error,
};

#[derive(Debug, Clone)]
pub enum GraphicsCoreState {
    Processing,
    Runnig,
    Waiting,
    Shutdown,
}

impl Default for GraphicsCoreState {
    fn default() -> Self {
        Self::Runnig
    }
}

pub struct GraphicsCore {
    state: GraphicsCoreState,
    custom_events: CustomEvents,
    logic_module_handler: LogicModuleHandler,
}

impl GraphicsCore {
    pub fn new(
        logic_module_handler: LogicModuleHandler,
        custom_events: CustomEvents,
    ) -> Self {
        Self { 
            state: GraphicsCoreState::default(), 
            custom_events: custom_events,
            logic_module_handler: logic_module_handler,
        }
    }
    pub fn on_event(
        &mut self, event: GraphicsEvent,
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
    ) {
        let current_state = std::mem::replace(
            &mut self.state, 
            GraphicsCoreState::Processing
        ); 

        self.state = match (current_state, event) {
            (GraphicsCoreState::Runnig, event) => {
                match event {
                    GraphicsEvent::CustomEvent(event) => {
                        match event {
                            CustomEvent::InternalEvent(event) => {
                                match GraphicsCoreLogic::internal_event_handle(
                                    event,
                                    graphics_backend,
                                    &mut self.logic_module_handler,
                                    ui
                                ) {
                                    Ok(Some(new_state)) => {
                                        new_state
                                    },
                                    Ok(None) => {
                                        GraphicsCoreState::Runnig        
                                    },
                                    Err(error) => {
                                        handle_graphic_event_error(error.into(), &self.custom_events);
                                        GraphicsCoreState::Runnig
                                    },
                                } 
                            },
                            CustomEvent::ExternalEvent(event) => {
                                match GraphicsCoreLogic::external_event_handle(
                                    event,
                                    ui
                                ) {
                                    Ok(Some(new_state)) => {
                                        new_state
                                    },
                                    Ok(None) => {
                                        GraphicsCoreState::Runnig        
                                    },
                                    Err(error) => {
                                        handle_graphic_event_error(error.into(), &self.custom_events);
                                        GraphicsCoreState::Runnig
                                    },
                                } 
                            },
                        } 
                    },
                    GraphicsEvent::WindowEvent(event) => {
                        match event {
                            WindowEvent::RedrawRequested => {
                                if let Err(error) = GraphicsCoreLogic::redraw_event_handle(
                                    graphics_backend,
                                    ui,
                                    &self.custom_events
                                ) {
                                    handle_graphic_event_error(error.into(), &self.custom_events);
                                }
                                 
                                GraphicsCoreState::Runnig
                            },
                            _ => {
                                match GraphicsCoreLogic::window_event_handle(
                                    event,
                                    graphics_backend
                                ) {
                                    Ok(Some(new_state)) => {
                                        new_state
                                    },
                                    Ok(None) => {
                                        GraphicsCoreState::Runnig        
                                    },
                                    Err(error) => {
                                        handle_graphic_event_error(error.into(), &self.custom_events);
                                        GraphicsCoreState::Runnig
                                    },
                                }
                            },
                        }  
                    },
                }
            }
            (GraphicsCoreState::Waiting, event) => {
                match event {
                    GraphicsEvent::WindowEvent(event) => {
                        match event {
                            WindowEvent::Resized(_) => {
                                if let Err(error) = GraphicsCoreLogic::window_event_handle(
                                    event,
                                    graphics_backend
                                ) {
                                    handle_graphic_event_error(error.into(), &self.custom_events);
                                }
                                GraphicsCoreState::Waiting
                            }
                            WindowEvent::RedrawRequested => {
                                if let Err(error) = GraphicsCoreLogic::redraw_event_handle(
                                    graphics_backend,
                                    ui,
                                    &self.custom_events
                                ) {
                                    handle_graphic_event_error(error.into(), &self.custom_events);
                                } 
                                GraphicsCoreState::Waiting
                            },
                            _ => GraphicsCoreState::Waiting
                        }
                    }
                    GraphicsEvent::CustomEvent(event) => {
                        match event {
                            CustomEvent::InternalEvent(event) => {
                                match event {
                                    InternalEvent::AppShutdownReq => {
                                        if let Err(error) = GraphicsCoreLogic::internal_event_handle(
                                            event,
                                            graphics_backend,
                                            &mut self.logic_module_handler,
                                            ui,
                                        ) {
                                            handle_graphic_event_error(error.into(), &self.custom_events);
                                        }
                                        GraphicsCoreState::Shutdown
                                    }
                                    _ => GraphicsCoreState::Waiting,
                                }
                            },
                            CustomEvent::ExternalEvent(event) => {
                                match GraphicsCoreLogic::external_event_handle(
                                    event,
                                    ui,
                                ) {
                                    Ok(Some(new_state)) => {
                                        new_state
                                    },
                                    Ok(None) => {
                                        GraphicsCoreState::Runnig        
                                    },
                                    Err(error) => {
                                        handle_graphic_event_error(error.into(), &self.custom_events);
                                        GraphicsCoreState::Runnig
                                    },
                                }
                            }
                        }
                    }
                }
            }
            (current_state,_) => current_state,
        }
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.state {
            GraphicsCoreState::Shutdown => true,
            _ => false,
        }
    }
}


pub enum GraphicsEventError {
    InternalEventError(InternalEventError),
    ExternalEventError(ExternalEventError),
    WindowEventError(WindowEventError),
    RedrawEventError(RedrawEventError),
}
