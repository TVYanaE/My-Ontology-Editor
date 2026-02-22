use crate::{
    modules::{
        logic_module::{
            events::{LogicCommand},
            logic_module_handler::LogicModuleHandler,
        },
    },
};
use tracing::{
    error
};
use super::{ 
    GraphicsCoreState,
    graphic_event_error::GraphicsEventError, 
};


pub fn graphic_event_error_handle(
    graphics_event_error: GraphicsEventError,
    logic_module_handler: &mut LogicModuleHandler,
) -> Option<GraphicsCoreState> { 
    match graphics_event_error {
        GraphicsEventError::SurfaceError(_) => {
            shutdown(logic_module_handler)        
        },
        GraphicsEventError::WGPUBackendError(_) => { 
            shutdown(logic_module_handler)        
        },
        GraphicsEventError::MPSCChannelError(_) => {
            shutdown(logic_module_handler)        
        },
        GraphicsEventError::EGUIBackendError(_) => {
            shutdown(logic_module_handler)        
        },
        GraphicsEventError::UIError(_) => {
            shutdown(logic_module_handler)        
        },
    } 
}

fn shutdown(
    logic_module_handler: &mut LogicModuleHandler,
) -> Option<GraphicsCoreState> {
    if let Err(_) = logic_module_handler.logic_commands.send(LogicCommand::Shutdown) {
        Some(GraphicsCoreState::Shutdown) 
    }
    else {
        if let Some(handle) = logic_module_handler.thread_handle.take() {
            if let Err(error) = handle.join() {        
                error!(error = ?error, "Logic Thread Panic");                
                Some(GraphicsCoreState::Shutdown)
            }
            else { 
                Some(GraphicsCoreState::Shutdown)
            }
        }
        else {         
            Some(GraphicsCoreState::Shutdown)
        }
    }
}
