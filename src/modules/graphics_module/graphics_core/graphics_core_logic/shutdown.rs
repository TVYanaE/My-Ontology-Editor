use std::{
    path::PathBuf,
};
use winit::{
    window::Window,
    dpi::PhysicalSize,
};
use tracing::{
    error,
};
use super::{
    GraphicsCoreLogic,
};
use crate::{
    modules::{
        logic_module::{
            events::{
                LogicCommand, 
                TaskID, TaskKind,
                TaskResult,
                DecisionKind, 
                ConfirmationID, ConfirmationKind,
            }, 
            logic_module_handler::LogicModuleHandler,
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            graphics_backend::{
                GraphicsBackend,
            },
            task_cache::{
                TaskCache, TaskContext,
            },
            ui::{UI, UIInputEvent, ChosedModalWindow},
            events::{
                CustomEvents
            },
        },  
    },
};


impl GraphicsCoreLogic {
    pub fn shutdown(
        logic_module_handler: &mut LogicModuleHandler,
    ) -> Option<GraphicsCoreState> {
        // logic for shutdown
        match logic_module_handler.logic_commands.send(LogicCommand::Shutdown) {
            Ok(_) => {
                if let Some(handle) = logic_module_handler.thread_handle.take() {
                    // Error will come due to panic in thread 
                    match handle.join() {
                        Ok(_) => {
                            Some(GraphicsCoreState::Shutdown)
                        },
                        Err(error) => {
                            error!(error = ?error, "Logic Thread Panic");                
                            Some(GraphicsCoreState::Shutdown)
                        }
                    }
                }
                else { 
                    Some(GraphicsCoreState::Shutdown)
                }
            },
            Err(error) => { 
                error!(error = ?error, "Logic Thread Panic");                
                Some(GraphicsCoreState::Shutdown)
            },
        } 
    }
}
