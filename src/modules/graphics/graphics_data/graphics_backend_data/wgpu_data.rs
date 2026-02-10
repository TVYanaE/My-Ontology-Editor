use std::{
    sync::Arc,
};
use winit::{
    dpi::PhysicalSize,
    window::Window,
};
use wgpu::{
    Instance,
    Adapter, 
    Device,  
    Queue, Surface, SurfaceConfiguration,
    TextureFormat,
};

pub struct WGPUData {
    pub window: Arc<Window>,
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub window_size: PhysicalSize<u32>,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub surface_texture_format: TextureFormat,
}
