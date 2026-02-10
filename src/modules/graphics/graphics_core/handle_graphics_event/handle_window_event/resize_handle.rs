use winit::{
    dpi::{
        PhysicalSize
    },
};
use wgpu::{
    SurfaceConfiguration, TextureUsages,
    CompositeAlphaMode, PresentMode
};
use crate::{
    modules::{
        graphics::{
            graphics_data::GraphicsData,
        },
    },
};

pub struct ResizeHandleContext<'c> {
    pub graphics_data: &'c mut GraphicsData, 
}

pub fn resize_handle(
    physical_size: PhysicalSize<u32>,
    resize_handle_context: ResizeHandleContext,
) {
    let wgpu_data = resize_handle_context
        .graphics_data
        .graphics_backend_data
        .wgpu_data
        .as_mut()
        .unwrap();

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
}
