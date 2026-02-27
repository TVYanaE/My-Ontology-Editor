use tracing::{
    error,
};
use crate::{
    modules::{
        db_module::{
            DBModuleHandler,
            DBCommand,
        },
    },
};
use super::{
    super::{
        logic_module_io::{
            event_sender::EventSender,
            logic_event::LogicEvent,
            event_manager::{EventManager, EventManagerError},
        }, 
    },
    LogicCoreState,
    logic_core_error::LogicCoreError,
}; 

pub fn logic_core_error_handle<S>(
    error: LogicCoreError<S>,
    event_manager: &EventManager<S>,
    db_module_handler: &mut DBModuleHandler,
) -> Option<LogicCoreState>  
where 
    S: EventSender
{
    match error {   
        LogicCoreError::STDIOError(_) => {
            graphic_module_shutdown(event_manager); 
            db_module_shutdown(db_module_handler);
            Some(LogicCoreState::Shutdown)
        },  
        LogicCoreError::ProjectManagerError(_) => {
            graphic_module_shutdown(event_manager); 
            db_module_shutdown(db_module_handler);
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::EventManagerError(_) => {
            db_module_shutdown(db_module_handler); 
            Some(LogicCoreState::Shutdown)
        },
        LogicCoreError::JobManagerError(_) => {
            graphic_module_shutdown(event_manager); 
            db_module_shutdown(db_module_handler);
            Some(LogicCoreState::Shutdown)
        },
    }
}

fn db_module_shutdown(
    db_module_handler: &mut DBModuleHandler
) {
    match db_module_handler.db_commands.send(DBCommand::Shutdown) {
        Ok(_) => {
            if let Some(handle) = db_module_handler.thread_handle.take() {
                match handle.join() {
                    Ok(_) => {}, 
                    Err(error) => {
                        error!(error = ?error, "Data Base Thread Panic");                
                    },
                }
            } 
        },
        Err(error) => { 
            error!(error = ?error, "Data Base Thread Panic");                
        },
    }
}

fn graphic_module_shutdown<S: EventSender>(
    event_manager: &EventManager<S>,
) {
    match event_manager.send_event(LogicEvent::Shutdown) {
        Ok(_) => {},
        Err(error) => {
            match error {
                EventManagerError::EventSenderError(error) => {
                    error!(error=?error, "Graphic Thread panic");
                },
            }
        },
    }
}
