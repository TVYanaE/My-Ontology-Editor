use egui_file_dialog::{
    FileDialog
};
use crate::{
    modules::{
        graphics::{
            graphics_states::{
                ui_state::{
                    create_new_project_window_state::CreateNewProjectWindowState,
                },
            },
        },
    },
};

pub struct CreateNewProjectWindowData {
    pub project_path: String,
    pub project_name: String,
    pub file_dialog: FileDialog,
    pub prev_state: Option<CreateNewProjectWindowState>,
}

impl Default for CreateNewProjectWindowData {
    fn default() -> Self {
        Self { 
            project_path: String::with_capacity(64), 
            project_name: String::with_capacity(32),
            file_dialog: FileDialog::new().as_modal(true).resizable(true),
            prev_state: None,
        }
    }
}
