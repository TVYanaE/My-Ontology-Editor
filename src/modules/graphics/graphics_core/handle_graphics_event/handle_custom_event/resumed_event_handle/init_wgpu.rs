use std::{
    sync::Arc,
};
use winit::{
    window::Window,
};
use wgpu::{
    Instance, InstanceDescriptor, 
    RequestAdapterOptions,
    DeviceDescriptor,
    TextureFormat, TextureUsages,
    SurfaceConfiguration,
    CompositeAlphaMode, PresentMode, 
};
use super::{
    CustomEventError,
};
use crate::{
    modules::{
        graphics::{
            graphics_data::{
                graphics_backend_data::{
                    wgpu_data::WGPUData,
                }, 
            },
            graphics_states::{
                graphics_backend_state::{
                    WGPUState,
                },
            },
        },
    },
};

pub struct InitWGPUContext<'c> {
    pub wgpu_state: &'c mut WGPUState,
    pub wgpu_data: &'c mut Option<WGPUData>,
    pub window: Window,
}

pub fn init_wgpu(
    init_wgpu_context: InitWGPUContext,    
) -> Result<(), CustomEventError> {
    let current_wgpu_state = std::mem::replace(
        init_wgpu_context.wgpu_state, 
        WGPUState::Processing
    );

    *init_wgpu_context.wgpu_state = match current_wgpu_state {
        WGPUState::NotInit => {
            let wgpu_data = create_wgpu_data(init_wgpu_context.window)?;
            *init_wgpu_context.wgpu_data = Some(wgpu_data); 

            WGPUState::Init
        },
        _ => current_wgpu_state
    };
    Ok(())
}

fn create_wgpu_data(window: Window) -> Result<WGPUData, CustomEventError> {
    let window = Arc::new(window);
    
    let instance_descriptor = InstanceDescriptor::default(); 

    let instance = Instance::new(&instance_descriptor);

    let request_adapter_options = RequestAdapterOptions::default(); 

    let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options)).unwrap();

    //let required_features = Features::TEXTURE_COMPRESSION_BC;

    let device_descriptor = DeviceDescriptor {
        //required_features: required_features,
        ..Default::default()
    };

    let (device, queue) = pollster::block_on(adapter.request_device(&device_descriptor))?;
    
    let surface = instance.create_surface(window.clone())?;

    let window_size = window.inner_size();
    let surface_capabilities = surface.get_capabilities(&adapter); 

    let selected_format = TextureFormat::Rgba8Unorm;
    let surface_texture_format = surface_capabilities
        .formats
        .iter()
        .find(|texture_format| **texture_format == selected_format)
        .ok_or_else(||{
            CustomEventError::TextureFormatIsntSupported
        })?;

    let surface_configuration = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface_texture_format.clone(),
        view_formats: vec![surface_texture_format.add_srgb_suffix()],
        alpha_mode: CompositeAlphaMode::Auto,
        width: window_size.width,
        height: window_size.height,
        desired_maximum_frame_latency: 2,
        present_mode: PresentMode::AutoVsync,
    };
    surface.configure(&device, &surface_configuration);

    let wgpu_data = WGPUData {
        window: window,
        device: device,
        surface: surface,
        surface_configuration: surface_configuration,
        window_size: window_size,
        queue: queue,
        adapter: adapter,
        instance: instance,
        surface_texture_format: *surface_texture_format
    };

    Ok(wgpu_data)
}
