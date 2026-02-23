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
                GraphicsEvent, CustomEvent, CustomEvents,
                InternalEvent, ExternalEvent,
            },
            graphics_backend::{
                GraphicsBackend
            },
            ui::UI,
        },
        logic_module::{
            logic_module_handler::LogicModuleHandler,
        },
    },
};

use super::{
    GraphicCoreStateHandle
}; 

pub struct RunningStateContext<'c> {
    pub graphics_backend: &'c mut GraphicsBackend,
    pub ui: &'c mut UI,
    pub logic_module_handler: &'c mut LogicModuleHandler,
    pub custom_events: &'c CustomEvents
}

impl GraphicCoreStateHandle {
    #[instrument(skip_all,err)]
    pub fn running_state_handle(
        context: RunningStateContext,
        event: GraphicsEvent, 
    ) -> Result<Option<GraphicsCoreState>, GraphicsCoreError> {
        match event {
            GraphicsEvent::CustomEvent(event) => {
                match event {
                    CustomEvent::InternalEvent(event) => {
                        match event {
                            InternalEvent::ShutdownReq => {
                                let new_state = GraphicsCoreLogic::app_shutdown_handle(
                                    context.logic_module_handler,
                                );

                                Ok(new_state)
                            },
                            InternalEvent::ResumedEvent(window) => {
                                let new_state = GraphicsCoreLogic::resumed_event_handle(
                                    context.graphics_backend, 
                                    window
                                )?;

                                Ok(new_state)
                            },
                            InternalEvent::CreateProjectReq { project_name, project_path } => {
                                let new_state = GraphicsCoreLogic::create_project_req_handle(
                                    context.logic_module_handler, 
                                    context.ui, 
                                    project_name, 
                                    project_path
                                )?;

                                Ok(new_state)
                            },
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
                            _ => Ok(None)
                        } 
                    },
                } 
            },
            GraphicsEvent::WindowEvent(event) => {
                match event {
                    WindowEvent::RedrawRequested => {
                        let new_state = GraphicsCoreLogic::redraw_event_handle(
                            context.graphics_backend,
                            context.ui,
                            context.custom_events
                        )?; 
                         
                        Ok(new_state)
                    },
                    WindowEvent::Resized(physical_size) => {
                        let new_state = GraphicsCoreLogic::resize_handle(
                            physical_size, 
                            context.graphics_backend
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
                            _ => { 

                                Ok(None)
                            },
                        }
                    },
                }
            },
        }   
    } 
}
