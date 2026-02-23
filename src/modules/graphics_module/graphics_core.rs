mod graphic_core_error;
mod graphic_core_state;
mod graphic_core_state_handle;
mod graphics_core_error_handle;
mod graphics_core_logic;

use crate::{
    modules::{ 
        logic_module::{
            logic_module_handler::LogicModuleHandler,
        },
    },
};
use super::{ 
    events::{
        CustomEvents, GraphicsEvent, 
    },
    graphics_backend::{ 
        GraphicsBackend,
    },
    ui::UI,
};
use self::{
    graphic_core_state_handle::{
        GraphicCoreStateHandle,
        RunningStateContext,
        WaitingTaskStateContext, 
    }, 
    graphics_core_error_handle::graphic_core_error_handle,
    graphic_core_state::GraphicsCoreState,
};


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
        &mut self, 
        event: GraphicsEvent,
        graphics_backend: &mut GraphicsBackend,
        ui: &mut UI,
    ) {
        let current_state = std::mem::replace(
            &mut self.state, 
            GraphicsCoreState::Processing
        ); 

        self.state = match (current_state, event) {
            (GraphicsCoreState::Runnig, event) => {
                match GraphicCoreStateHandle::running_state_handle(
                    RunningStateContext { 
                        graphics_backend, 
                        ui, 
                        logic_module_handler: &mut self.logic_module_handler,
                        custom_events: &self.custom_events
                    }, 
                    event
                ) {
                    Ok(Some(new_state)) => {
                        new_state
                    },
                    Ok(None) => {
                        GraphicsCoreState::Runnig        
                    },
                    Err(error) => {
                        if let Some(new_state) = graphic_core_error_handle(
                            error.into(), &mut self.logic_module_handler
                        ) {
                            new_state 
                        }
                        else {       
                            GraphicsCoreState::Runnig
                        }
                    },
                }         
            }
            (GraphicsCoreState::WaitingTask { 
                task_id, 
            }, event) => {
                match GraphicCoreStateHandle::waiting_task_state_handle(
                    WaitingTaskStateContext {
                        graphics_backend: graphics_backend,
                        ui: ui,
                        custom_events: &self.custom_events,
                        logic_module_handler: &mut self.logic_module_handler,
                        waiting_task_id: task_id.clone(),
                    },
                    event
                ) {
                    Ok(Some(new_state)) => {
                        new_state
                    },
                    Ok(None) => {
                        GraphicsCoreState::WaitingTask{task_id: task_id} 
                    },
                    Err(error) => {
                        if let Some(new_state) = graphic_core_error_handle(
                            error.into(), 
                            &mut self.logic_module_handler,
                        ) {
                            new_state
                        }
                        else {       
                            GraphicsCoreState::Runnig
                        }
                    },
                }
            }, 
            (current_state,_) => current_state, 
        };
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.state {
            GraphicsCoreState::Shutdown => true,
            _ => false,
        }
    }
}
