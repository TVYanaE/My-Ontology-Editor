use crate::{
    modules::{
        logic_module::{
            prelude::{
                logic_command::LogicCommand,
                LogicModuleHandler
            },
        },
        graphics_module::{
            graphics_core::{
                graphic_core_error::{
                    GraphicsCoreError
                },
                graphic_core_state::GraphicsCoreState,
            },
        },
    },
};
use tracing::{
    error
};

pub fn graphic_core_error_handle(
    graphics_core_error: GraphicsCoreError,
    logic_module_handler: &mut LogicModuleHandler,
) -> Option<GraphicsCoreState> { 
    match graphics_core_error {
        GraphicsCoreError::SurfaceError(_) => {
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
        GraphicsCoreError::WGPUBackendError(_) => { 
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
        GraphicsCoreError::MPSCChannelError(_) => {
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
        GraphicsCoreError::EGUIBackendError(_) => {
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
        GraphicsCoreError::UIError(_) => {
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
        GraphicsCoreError::TaskContextNotFound => {
            shutdown(logic_module_handler); 
            Some(GraphicsCoreState::Shutdown)
        },
    } 
}

fn shutdown(
    logic_module_handler: &mut LogicModuleHandler,
) {
    match logic_module_handler.logic_commands.send(LogicCommand::Shutdown) {
        Ok(_) => {
            if let Some(handle) = logic_module_handler.thread_handle.take() {
                match handle.join() {
                    Ok(_) => {},
                    Err(error) => {
                        error!(error = ?error, "Logic Thread Panic"); 
                    },
                }     
            } 
        },
        Err(error) => { 
            error!(error = ?error, "Logic Thread Panic"); 
        },
    };  
}
