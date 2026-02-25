use tracing::{
    instrument,
};
use winit::{
    event::WindowEvent,
};
use crate::{
    modules::{
        graphics_module::{
            graphics_core::{
                graphics_core_logic::{
                    GraphicsCoreLogic
                },             
                graphic_core_error::GraphicsCoreError,
                GraphicsCoreState,
            },
            events::{
                ExternalEvent, CustomEvents,
                GraphicsEvent, CustomEvent, 
                InternalEvent, 
            },
            graphics_backend::{
                GraphicsBackend
            },
            task_cache::TaskCache,
            ui::UI,
        },
        logic_module::{
            logic_module_handler::LogicModuleHandler,
            events::TaskID,
        },
    },
};
use super::{
    GraphicCoreStateHandle,
};

pub struct WaitingTaskStateContext<'c> {
    pub graphics_backend: &'c mut GraphicsBackend,
    pub ui: &'c mut UI,
    pub custom_events: &'c CustomEvents,
    pub logic_module_handler: &'c mut LogicModuleHandler,
    pub waiting_task_id: TaskID,
    pub task_cache: &'c mut TaskCache,
}

impl GraphicCoreStateHandle {
    #[instrument(skip_all,err)]
    pub fn waiting_task_state_handle(
        context: WaitingTaskStateContext,
        event: GraphicsEvent,
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        match event {
            GraphicsEvent::WindowEvent(event) => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        let new_state = GraphicsCoreLogic::resize_handle(
                            physical_size, 
                            context.graphics_backend
                        )?;
                        
                        Ok(new_state) 
                    }
                    WindowEvent::RedrawRequested => {
                        let new_state = GraphicsCoreLogic::redraw_event_handle(
                            context.graphics_backend,
                            context.ui,
                            context.custom_events
                        )?;

                        Ok(new_state)
                    },
                    WindowEvent::CloseRequested => {
                        let new_state = GraphicsCoreLogic::app_shutdown_handle(
                            context.logic_module_handler,
                        );

                        Ok(new_state)
                    },
                    _ => {
                        let wgpu_data = context.graphics_backend.wgpu_backend.get_wgpu_data()?;
                        let resp = context.graphics_backend.egui_backend.on_window_event(&event, wgpu_data)?;

                        if resp.repaint {
                            wgpu_data.window.request_redraw();
                        }

                        if resp.consumed {
                            return Ok(None);
                        }

                        match event {
                            _ => Ok(None)
                        }    
                    }, 
                }  
            },
            GraphicsEvent::CustomEvent(event) => {
                match event {
                    CustomEvent::InternalEvent(event) => {
                        match event {
                            InternalEvent::ShutdownReq => {
                                let new_state = GraphicsCoreLogic::app_shutdown_handle(
                                    context.logic_module_handler,
                                );

                                Ok(new_state) 
                            }
                            InternalEvent::ConfirmationDecision { 
                                confirmation_id, 
                                decision,
                                decision_kind,
                            } => {
                                let new_state = GraphicsCoreLogic::confirmation_obtain_handle(
                                    confirmation_id, 
                                    decision,
                                    decision_kind,
                                    context.logic_module_handler
                                )?;

                                Ok(new_state)
                            },
                            _ => Ok(None),
                        }
                    },
                    CustomEvent::ExternalEvent(event) => {
                        match event {
                            ExternalEvent::Shutdown => {
                                let new_state = GraphicsCoreLogic::app_shutdown_handle(
                                    context.logic_module_handler,
                                );

                                Ok(new_state) 
                            },
                            ExternalEvent::TaskRespone { 
                                task_id,
                                task_result,
                            } => {
                                let new_state = GraphicsCoreLogic::task_response_handle(
                                    context.waiting_task_id, 
                                    task_id, 
                                    task_result, 
                                    context.ui,
                                    context.task_cache,
                                )?;

                                Ok(new_state)
                            }, 
                            ExternalEvent::ConfirmationRequested { 
                                confirmation_id, 
                                confirmation_kind 
                            } => {
                                let new_state = GraphicsCoreLogic::confirmation_required_handle(
                                    context.ui, 
                                    confirmation_id, 
                                    confirmation_kind
                                )?;

                                Ok(new_state)
                            },
                        }
                    }
                }
            }
        }
    }
} 

