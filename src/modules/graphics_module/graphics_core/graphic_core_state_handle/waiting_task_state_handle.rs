
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
                pending_task::PendingTask,
                graphic_event_error::GraphicsEventError,
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
            ui::UI,
        },
        shared::{
            logic_module_handler::LogicModuleHandler,
            task_id::TaskID,
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
    pub pending_task_id: TaskID,
    pub pending_task: PendingTask,
}

impl GraphicCoreStateHandle {
    pub fn waiting_task_state_handle(
        context: WaitingTaskStateContext,
        event: GraphicsEvent,
    ) -> Result<Option<GraphicsCoreState>, GraphicsEventError> {
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
                            context.logic_module_handler
                        )?;

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
                            InternalEvent::AppShutdownReq => {
                                let new_state = GraphicsCoreLogic::app_shutdown_handle(
                                    context.logic_module_handler
                                )?;

                                Ok(new_state) 
                            }
                            InternalEvent::ConfirmationObtain { task_id, confirm } => {
                                let new_state = GraphicsCoreLogic::confirmation_obtain_handle(
                                    task_id, 
                                    confirm, 
                                    context.logic_module_handler
                                )?;

                                Ok(new_state)
                            },
                            _ => Ok(None),
                        }
                    },
                    CustomEvent::ExternalEvent(event) => {
                        match event {
                            ExternalEvent::AppShutdownReq => {
                                let new_state = GraphicsCoreLogic::app_shutdown_handle(
                                    context.logic_module_handler
                                )?;

                                Ok(new_state) 
                            },
                            ExternalEvent::TaskDone(done_task_id) => {
                                println!("Task Done");
                                let new_state = GraphicsCoreLogic::pending_task_handle(
                                    context.pending_task_id, 
                                    context.pending_task, 
                                    done_task_id, 
                                    context.ui
                                )?;

                                Ok(new_state)
                            },
                            ExternalEvent::ConfirmRequeired { task_id, text } => {
                                let new_state = GraphicsCoreLogic::confirmation_required_handle(
                                    context.ui, 
                                    task_id, 
                                    &text
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

