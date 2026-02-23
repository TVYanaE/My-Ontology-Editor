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
        events::{EventSender, LogicEvent}, 
    },
    LogicCoreState,
    logic_core_error::LogicCoreError,
}; 

pub fn logic_core_error_handle<S>(
    error: LogicCoreError<S>,
    sender: &S,
    db_module_handler: &mut DBModuleHandler,
) -> Option<LogicCoreState>  
where 
    S: EventSender
{
    match error { 
        LogicCoreError::EventSenderError(_) => {
            db_module_shutdown(db_module_handler); 
            Some(LogicCoreState::Shutdown)
        },  
        LogicCoreError::STDIOError(_) => {
            logic_module_shutdown(sender); 
            db_module_shutdown(db_module_handler);
            Some(LogicCoreState::Shutdown)
        },  
        LogicCoreError::ProjectManagerError(_) => {
            logic_module_shutdown(sender); 
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

fn logic_module_shutdown<S: EventSender>(
    sender: &S,
) {
    match sender.send_event(LogicEvent::Shutdown) {
        Ok(_) => {},
        Err(error) => {
            error!(error=?error, "Graphic Thread panic");
        },
    }
}
