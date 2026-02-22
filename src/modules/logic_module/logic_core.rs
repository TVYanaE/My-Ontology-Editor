mod logic_core_error_handle;
mod logic_core_logic;
mod logic_core_state;
mod logic_core_state_handle;

use thiserror::{
    Error,
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories,
        logic_module::{
            events::{
                LogicCommand, EventSender,
            },
        },
        db_module::DBEvent, 
    },
};
use self::{
    logic_core_error_handle::logic_core_error_handle,
    logic_core_state::LogicCoreState,
    logic_core_state_handle::LogicCoreStateHandle,
};

pub struct LogicCore {
    logic_core_state: LogicCoreState,  
}

#[derive(Debug, Error)]
pub enum LogicCoreError<S: EventSender>{ 
    #[error("MPSC Channel was closed {0}")]
    MPSCChannelDBEventError(#[from] std::sync::mpsc::SendError<DBEvent>),

    #[error("Event Sender Error: {0}")]
    EventSenderError(#[source] S::Error),

    #[error("Std IO Error: {0}")]
    STDIOError(#[from] std::io::Error), 
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
        app_dirs: &ApplicationDirectories, 
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
                    app_dirs, 
                    event_sender
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Ready,
                    Err(error) => {
                        if let Some(new_state) = logic_core_error_handle(error, event_sender) {
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
                    app_dirs,
                    event_sender,
                    work_after_confirmation,
                    confirmation_id,
                ) {
                    Ok(Some(new_state)) => new_state,
                    Ok(None) => LogicCoreState::Ready,
                    Err(error) => {
                        if let Some(new_state) = logic_core_error_handle(error, event_sender) {
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
