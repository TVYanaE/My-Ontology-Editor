pub mod graphics_backend_data;
pub mod ui_data;

use std::{
    sync::{
        Arc
    }
};
use crate::{
    modules::{
        app_dirs::ApplicationDirectories
    },
};
use self::{
    graphics_backend_data::GraphicsBackendData,
    ui_data::UIData,
};

pub struct GraphicsData {
    pub graphics_backend_data: GraphicsBackendData,
    pub ui_data: UIData,
    pub app_dirs: Arc<ApplicationDirectories>,
}

impl GraphicsData {
    pub fn new(app_dirs: Arc<ApplicationDirectories>) -> Self {
        Self { 
            graphics_backend_data: GraphicsBackendData::default(), 
            ui_data: UIData::default(), 
            app_dirs: app_dirs 
        }
    }
}

