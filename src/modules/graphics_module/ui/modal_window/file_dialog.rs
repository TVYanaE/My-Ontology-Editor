use egui_file_dialog::{
    FileDialog,
    DialogState,
};
use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics_module::{
            ui::{
                events::{UIEvents, UIEvent},
                ui_error::UIError,
            },
        },
    },
};


pub struct FileDialogWrap {
    file_dialog: FileDialog
}

impl Default for FileDialogWrap {
    fn default() -> Self {
        Self { 
            file_dialog: FileDialog::new().resizable(true).movable(true).as_modal(true) 
        }
    }
}

impl FileDialogWrap {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext
    ) -> Result<UIEvents, UIError> {
        let mut ui_events = UIEvents::with_capacity(4);

        prepare_file_dialog(&mut self.file_dialog, egui_context, &mut ui_events);    

        Ok(ui_events)
    }

    pub fn open_for_pick_directory(&mut self) {
        self.file_dialog.pick_directory();
    }
}

 fn prepare_file_dialog(
    file_dialog: &mut FileDialog,
    egui_context: &EGUIContext,
    ui_events: &mut Vec<UIEvent>,
) {
    file_dialog.update(egui_context);

    if let Some(project_path) = file_dialog.picked() {
        if let Some(project_path_str) = project_path.to_str() {
            ui_events.push(UIEvent::DirPicked(project_path_str.to_string()));              
        }
        else {
            ui_events.push( 
                UIEvent::ShowNotification(
                    "Invalid Symbols was found in path. Please Use UTF-8 Symbols Only".to_string()
                )
            ); 
        }
    }
    else {
        // For close and cancel handling
        match file_dialog.state() {
            DialogState::Closed | DialogState::Cancelled => {
                ui_events.push(UIEvent::FileDialogClosed); 
            },
            _ => {}
        }
    } 
}
