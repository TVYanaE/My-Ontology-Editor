mod wgpu_backend_logic;

use std::{
    sync::Arc,
};
use thiserror::{
    Error,
};
use winit::{
    dpi::PhysicalSize,
    window::Window,
};
use wgpu::{
    Instance, Adapter, Device, Queue, Surface, 
    SurfaceConfiguration, TextureFormat,
};
use self::{
    wgpu_backend_logic::WGPUBackendLogic
};

#[derive(Debug)]
pub enum WGPUState {
    NotInit,
    Init,
    Processing,
}

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

impl Default for WGPUState {
    fn default() -> Self {
        Self::NotInit
    }
}

#[derive(Debug, Error, Clone)]
pub enum WGPUBackendError {
    #[error("Request Device Error: {0} ")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),

    #[error("Request Adapter Error: {0}")]
    RequestAdapterError(#[from] wgpu::RequestAdapterError),

    #[error("Create Surface Error: {0}")]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    #[error("Choosed Texture Format isn't supported")]
    TextureFormatIsntSupported,

    #[error("WGPU Backend wasn't initialised")]
    WGPUBackendWasntInit,
}

#[derive(Default)]
pub struct WGPUBackend {
    state: WGPUState,
    data: Option<WGPUData>,
}

impl WGPUBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, window: Window) -> Result<(), WGPUBackendError> {
        let current_state = std::mem::replace(
            &mut self.state, 
            WGPUState::Processing
        );

        self.state = match current_state {
            WGPUState::NotInit => {
                let wgpu_data = WGPUBackendLogic::init_wgpu_data(window)?;
                self.data = Some(wgpu_data);

                WGPUState::Init
            }
            _ => current_state,
        };

        Ok(())
    }
    
    pub fn resize(&mut self, physical_size: PhysicalSize<u32>) -> Result<(), WGPUBackendError> {
        let wgpu_data = self
            .data
            .as_mut()
            .ok_or_else(||{
                WGPUBackendError::WGPUBackendWasntInit
            })?;

        WGPUBackendLogic::resize(wgpu_data, physical_size);
        Ok(())
    }

    pub fn get_wgpu_data(&self) -> Result<&WGPUData, WGPUBackendError> {
        let data = self.data
            .as_ref()
            .ok_or_else(||{
                WGPUBackendError::WGPUBackendWasntInit
            })?;
        Ok(data)
    }
}
