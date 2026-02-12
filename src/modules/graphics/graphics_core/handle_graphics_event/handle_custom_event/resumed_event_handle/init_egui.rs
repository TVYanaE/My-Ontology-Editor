use crate::{
    aliases::{
        EGUIContext, EGUIWinitState, EGUIRendererOptions,
        EGUIRenderer,
    },
    modules::{
        graphics::{
            graphics_data::{
                graphics_backend_data::{
                    wgpu_data::WGPUData,
                    egui_data::EGUIData,
                },
            },
            graphics_states::{
                graphics_backend_state::EGUIState,
            },
        },
    },
};

pub struct InitEGUIContext<'c> {
    pub egui_state: &'c mut EGUIState,
    pub wgpu_data: &'c Option<WGPUData>,
    pub egui_data: &'c mut Option<EGUIData>,
}

pub fn init_egui(
    init_egui_context: InitEGUIContext,
) {
    let wgpu_data = init_egui_context.wgpu_data.as_ref().unwrap();

    let current_egui_state = std::mem::replace(
        init_egui_context.egui_state, 
        EGUIState::Processing
    );

    *init_egui_context.egui_state = match current_egui_state {
        EGUIState::NotInit => {
            let egui_data = create_egui_data(wgpu_data);
            *init_egui_context.egui_data = Some(egui_data);

            EGUIState::Init
        },
        _ => current_egui_state
    };
}

fn create_egui_data(
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

    return egui_data;
}
