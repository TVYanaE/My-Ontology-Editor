pub struct EGUIBackendLogic;

use egui::FullOutput;
use winit::{
    event::WindowEvent,
};
use crate::{
    aliases::{
        EGUIContext, EGUIWinitState, EGUIRendererOptions,
        EGUIRenderer, EGUIEventRespone,
    },
    modules::{
        graphics_module::{
            graphics_backend::{
                wgpu_backend::WGPUData
            },
            graphics_core::{
                graphics_event::{
                    InternalEvent,
                    CreateProjectDescriptor,
                },
            },
            ui::{
                UI, UIAffect,
            },
        },
    },
};
use super::{
    CustomEvents, EGUIData,
};

impl EGUIBackendLogic {
    pub fn init(
        wgpu_data: &WGPUData
    ) -> EGUIData {
        let max_texture_size = wgpu_data.device.limits().max_texture_dimension_2d as usize;

        let egui_context = EGUIContext::default();  

        let scale_factor = wgpu_data.window.scale_factor() as f32;
        let viewport_id = egui_context.viewport_id();

        let egui_winit_state = EGUIWinitState::new(
            egui_context.clone(), 
            viewport_id, 
            &wgpu_data.window, 
            Some(scale_factor), 
            Some(winit::window::Theme::Dark), 
            Some(max_texture_size),
        );

        let egui_renderer = EGUIRenderer::new(
            &wgpu_data.device, 
            wgpu_data.surface_texture_format, 
            EGUIRendererOptions::default(),
        ); 

        let egui_data = EGUIData {
            egui_renderer: egui_renderer,
            egui_winit_state: egui_winit_state,
        };
       
        let window_clone = wgpu_data.window.clone();

        egui_data.egui_winit_state.egui_ctx().set_request_repaint_callback(Box::new(move |_info|{
            window_clone.clone().request_redraw();
        })); 

        egui_data
    }

    pub fn prepare_ui(
        egui_data: &mut EGUIData,
        wgpu_data: &WGPUData,
        ui: &mut UI,
        custom_events: &CustomEvents
    ) -> FullOutput {
        let raw_input = egui_data
        .egui_winit_state
        .take_egui_input(&wgpu_data.window);

        let mut ui_affects = Vec::with_capacity(32); 

        let full_output = egui_data
            .egui_winit_state
            .egui_ctx()
            .run(raw_input, |egui_context|{
                let affects = ui.prepare_ui(egui_context);
                ui_affects.extend(affects);
            }); 

        for affect in ui_affects {
            match affect {
                UIAffect::ExitRequested => {
                    custom_events.send_event(InternalEvent::AppShutdownReq.into());
                },
                UIAffect::CreateProjectReq(req) => {
                    custom_events.send_event(InternalEvent::CreateProjectReq(
                        CreateProjectDescriptor { 
                            project_name: req.project_name.clone(), 
                            project_dir: req.project_dir.clone(),
                        }
                    ).into());
                },
            } 
        }

        full_output
    }

    pub fn window_event_handle(
        event: &WindowEvent,
        egui_data: &mut EGUIData,
        wgpu_data: &WGPUData, 
    ) -> EGUIEventRespone{
        let egui_response = egui_data
            .egui_winit_state.on_window_event(
                &wgpu_data.window, 
                &event
            ); 

        egui_response
    }
}
