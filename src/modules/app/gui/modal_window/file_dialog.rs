
use eframe::egui::Context as EGUIContext;
use egui_file_dialog::FileDialog as EGUIFileDialog;
use egui_file_dialog::DialogState;

use super::super::gui_state::ChoosingItemType;
use super::super::gui_state::FileDialogResponseReceiver;
use super::super::gui_event::{GUIEvent, GUIEventBuffer};

pub struct FileDialog {
    is_open: bool,
    file_dialog: EGUIFileDialog,
}

impl FileDialog {
    pub fn new() -> Self {
        Self { 
            is_open: false,
            file_dialog: EGUIFileDialog::new().as_modal(true),
        }
    } 
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        choosing_item_type: &ChoosingItemType,
        event_buffer: &mut GUIEventBuffer,
        receiver: &FileDialogResponseReceiver
    ) {
        if !self.is_open {
            match choosing_item_type {
                ChoosingItemType::Dir => {
                    self.file_dialog.pick_directory();
                    self.is_open = true;
                },
                ChoosingItemType::File => {
                    self.file_dialog.pick_file();
                    self.is_open = true;
                },
            }
        }

        self.file_dialog.update(context);

        if let Some(selected_path) = self.file_dialog.take_picked() {
            event_buffer.push(
                GUIEvent::PathSelected {
                    path: selected_path,
                    receiver: receiver.clone(),
                }
            );
            self.is_open = false;
        }

        match self.file_dialog.state() { 
            DialogState::Cancelled => {
                event_buffer.push(GUIEvent::FileDialogCanceled);
                self.is_open = false;
            },
            _ => {}, 
        }
    }
}
