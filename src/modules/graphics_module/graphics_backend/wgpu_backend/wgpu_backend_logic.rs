use std::{
    sync::Arc,
};

use winit::{
    dpi::PhysicalSize,
    window::Window,
};
use wgpu::{
    Instance, 
    SurfaceConfiguration, TextureFormat,
    InstanceDescriptor, 
    RequestAdapterOptions,
    DeviceDescriptor,
    TextureUsages,
    CompositeAlphaMode, PresentMode, 
};
use super::{
    WGPUData, WGPUBackendError
};

pub struct WGPUBackendLogic;

impl WGPUBackendLogic {
    pub fn init_wgpu_data(window: Window) -> Result<WGPUData, WGPUBackendError> {
        let window = Arc::new(window);
    
        let instance_descriptor = InstanceDescriptor::default(); 

        let instance = Instance::new(&instance_descriptor);

        let request_adapter_options = RequestAdapterOptions::default(); 

        let adapter = pollster::block_on(instance.request_adapter(&request_adapter_options))?;

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
                WGPUBackendError::TextureFormatIsntSupported
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

    pub fn resize(
        wgpu_data: &mut WGPUData,
        physical_size: PhysicalSize<u32>,
    ) { 
        wgpu_data.window_size = physical_size;
            
        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: wgpu_data.surface_texture_format,
            view_formats: vec![wgpu_data.surface_texture_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: wgpu_data.window_size.width,
            height: wgpu_data.window_size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        wgpu_data.surface.configure(&wgpu_data.device, &surface_configuration);
        wgpu_data.surface_configuration = surface_configuration;
        wgpu_data.window.request_redraw();
    } 
}
