mod egui_backend_logic;

use wgpu::wgc::error;
use winit::{
    event::WindowEvent,
};
use egui::FullOutput;
use thiserror::{
    Error
};
use crate::{
    aliases::{
        EGUIRenderer, EGUIWinitState,
        EGUIEventRespone,
    },
    modules::{
        graphics_module::{
            graphics_backend::{
                wgpu_backend::{
                    WGPUData
                },
            }, 
            ui::{UI, UIError}
        },
    },
};
use super::{
    super::{
        CustomEvents,
    },
};
use self::{
    egui_backend_logic::EGUIBackendLogic,
};

#[derive(Debug)]
pub enum EGUIState {
    Processing,
    NotInit,
    Init,
}


#[derive(Debug, Error, Clone)]
pub enum EGUIBackendError {
    #[error("EGUI Backend wasn't initialised")]
    EGUIBackendWasntInit,

    #[error("UI Error: {0}")]
    UIError(#[from] UIError),
}

impl Default for EGUIState {
    fn default() -> Self {
        Self::NotInit
    }
}

#[derive(Default)]
pub struct EGUIBackend {
    state: EGUIState,
    data: Option<EGUIData>,
}

pub struct EGUIData {
    pub egui_winit_state: EGUIWinitState,
    pub egui_renderer: EGUIRenderer,
}

impl EGUIBackend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(
        &mut self,
        wgpu_data: &WGPUData
    ) {
        let current_state = std::mem::replace(
            &mut self.state, 
            EGUIState::Processing
        );

        self.state = match current_state {
            EGUIState::NotInit => {
                let egui_data = EGUIBackendLogic::init(wgpu_data);
                self.data = Some(egui_data);

                EGUIState::Init
            },
            _ => current_state,
        };
    } 

    pub fn prepare_ui(
        &mut self,
        wgpu_data: &WGPUData, 
        ui: &mut UI,
        custom_events: &CustomEvents,
    ) -> Result<FullOutput, EGUIBackendError> {
        let egui_data = self
            .data
            .as_mut()
            .ok_or_else(||{
                EGUIBackendError::EGUIBackendWasntInit
            })?;

        let full_output = EGUIBackendLogic::prepare_ui(egui_data, wgpu_data, ui, custom_events)?;

        Ok(full_output)
    }

    pub fn on_window_event(
        &mut self,
        event: &WindowEvent,
        wgpu_data: &WGPUData
    ) -> Result<EGUIEventRespone, EGUIBackendError> {
        let egui_data = self
            .data
            .as_mut()
            .ok_or_else(||{
                EGUIBackendError::EGUIBackendWasntInit
            })?;

        let resp = EGUIBackendLogic::window_event_handle(event, egui_data, wgpu_data);

        Ok(resp)
    }

    pub fn get_mut_egui_data(&mut self) -> Result<&mut EGUIData, EGUIBackendError> {
        let egui_data = self
            .data
            .as_mut()
            .ok_or_else(||{
                EGUIBackendError::EGUIBackendWasntInit
            })?;

        Ok(egui_data)
    }
}
