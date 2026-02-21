mod graphic_core_state_handle;
mod graphic_event_error;
mod graphics_core_logic;
mod graphics_event_error_handle;
mod pending_task;


use crate::{
    modules::{ 
        shared::{
            logic_module_handler::LogicModuleHandler,
            task_id::TaskID,
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
    pending_task::PendingTask,
    graphics_event_error_handle::graphic_event_error_handle,
};

#[derive(Debug, Clone)]
pub enum GraphicsCoreState {
    Processing,
    Runnig,
    WaitingTask {
        task_id: TaskID, 
        pending_task: PendingTask
    },
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
                        graphic_event_error_handle(error.into(), &self.custom_events);
                        GraphicsCoreState::Runnig
                    },
                }         
            }
            (GraphicsCoreState::WaitingTask { 
                task_id, 
                pending_task 
            }, event) => {
                match GraphicCoreStateHandle::waiting_task_state_handle(
                    WaitingTaskStateContext {
                        graphics_backend: graphics_backend,
                        ui: ui,
                        custom_events: &self.custom_events,
                        logic_module_handler: &mut self.logic_module_handler,
                        pending_task: pending_task.clone(),
                        pending_task_id: task_id.clone()
                    },
                    event
                ) {
                    Ok(Some(new_state)) => {
                        new_state
                    },
                    Ok(None) => {
                        GraphicsCoreState::WaitingTask{task_id: task_id, pending_task: pending_task} 
                    },
                    Err(error) => {
                        graphic_event_error_handle(error.into(), &self.custom_events);
                        GraphicsCoreState::Runnig
                    },
                }
            }, 
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


