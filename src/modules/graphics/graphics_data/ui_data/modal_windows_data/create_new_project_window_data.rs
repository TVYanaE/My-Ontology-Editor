use egui_file_dialog::{
    FileDialog
};

pub struct CreateNewProjectWindowData {
    pub project_path: String,
    pub file_dialog: FileDialog,
}

impl Default for CreateNewProjectWindowData {
    fn default() -> Self {
        Self { 
            project_path: String::with_capacity(64), 
            file_dialog: FileDialog::new().as_modal(true).resizable(true),
        }
    }
}
