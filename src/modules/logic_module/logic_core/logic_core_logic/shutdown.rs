use tracing::{
    error,
};
use crate::{
    modules::{
        db_module::{
            DBModuleHandler, DBCommand,
        },
    },
};
use super::{
    super::{
        super::{
            job_manager::{
                Job,
            },
        }, 
    },
    LogicCoreLogic 
};

impl LogicCoreLogic {
    pub fn shutdown(
        db_module_handler: &mut DBModuleHandler 
    ) -> Vec<Job> {
        let jobs = Vec::with_capacity(2);
        match db_module_handler.db_commands.send(DBCommand::Shutdown) {
            Ok(_) => {
                if let Some(handle) = db_module_handler.thread_handle.take() {
                    match handle.join() {
                        Ok(_) => {
                        }, 
                        Err(error) => {
                            error!(error = ?error, "Data Base Thread Panic");                
                        },
                    }
                }
            },
            Err(error) => { 
                error!(error = ?error, "Data Base Thread Panic");                
            },
        };
        jobs 
    } 
}
