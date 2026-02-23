mod logic_core_error;
mod logic_core_error_handle;
mod logic_core_logic;
mod logic_core_state;
mod logic_core_state_handle;

use crate::{
    modules::{
        db_module::DBModuleHandler,
    },
};
use super::{
    events::{
        LogicCommand, EventSender,
    },
    project_manager::{
        ProjectManager,
    },
};
use self::{
    logic_core_error_handle::logic_core_error_handle,
    logic_core_state::LogicCoreState,
    logic_core_state_handle::{
        LogicCoreStateHandle,
        ReadyStateContext, WaitingConfirmationStateContext,
    },
};

pub struct LogicCore {
    logic_core_state: LogicCoreState,  
}

impl LogicCore {
    pub fn new() -> Self {
        Self {
            logic_core_state: LogicCoreState::default(),
        }
    }
    
    pub fn on_command<S: EventSender>(
        &mut self, 
        command: LogicCommand,
        db_module_handler: &mut DBModuleHandler,
        project_manager: &ProjectManager,
        event_sender: &S,
    ) {
        let current_state = std::mem::replace(
            &mut self.logic_core_state, 
            LogicCoreState::Processing
        ); 

        self.logic_core_state = match (current_state, command) {
            (LogicCoreState::Ready, command) => {
                match LogicCoreStateHandle::ready_handle(
                    command, 
                    ReadyStateContext { 
                        event_sender: event_sender, 
                        project_manager: project_manager, 
                        db_module_handler: db_module_handler,
                    }
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Ready,
                    Err(error) => {
                        if let Some(new_state) = logic_core_error_handle(
                            error, 
                            event_sender,
                            db_module_handler,
                        ) {
                            new_state
                        } 
                        else {
                            LogicCoreState::Ready
                        } 
                    },
                }              
            },
            (LogicCoreState::WaitConfirmation { 
                confirmation_id, 
                work_after_confirmation 
            }, command) => {
                match LogicCoreStateHandle::waiting_confirmation_handle(
                    command, 
                    WaitingConfirmationStateContext { 
                        event_sender: event_sender, 
                        work: work_after_confirmation, 
                        waiting_confirmation_id: confirmation_id, 
                        project_manager: project_manager,
                        db_module_handler: db_module_handler,
                    } 
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Ready,
                    Err(error) => {
                        if let Some(new_state) = logic_core_error_handle(
                            error, 
                            event_sender,
                            db_module_handler,
                        ) {
                            new_state
                        } 
                        else {
                            LogicCoreState::Ready
                        } 
                    },
                }
            },
            (current_state,_) => current_state,
        }
    }

    pub fn event_loop_closed_handle(&mut self) {
        self.logic_core_state = LogicCoreState::Shutdown;
    }

    pub fn is_shutdown(&self) -> bool {
        match &self.logic_core_state {
            LogicCoreState::Shutdown => true,
            _ => false
        }
    }
}
