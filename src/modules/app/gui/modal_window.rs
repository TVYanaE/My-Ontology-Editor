mod confirmation_window;
pub mod create_project_window;
mod file_dialog;
mod notification;

use eframe::egui::Context as EGUIContext;

use super::gui_event::GUIEventBuffer;
use super::gui_command::ConfirmationType;

use self::confirmation_window::ConfirmationWindow;
use self::create_project_window::CreateProjectWindow;
use self::file_dialog::FileDialog;
use self::notification::Notification;

#[derive(Debug, Clone)]
pub enum ModalWindowType {
    CreateProjectWindow, 
    FileDialog(ChoosingItemType),
    Notification(String),
    ConfirmationWindow {
        confirmation_text: String, 
        confirmation_type: ConfirmationType,
    },
}

#[derive(Debug, Clone)]
pub enum ChoosingItemType {
    File,
    Dir,
}

pub struct ModalWindow {
    create_project_window: CreateProjectWindow,
    confirmation_window: ConfirmationWindow,
    file_dialog: FileDialog,
    notification: Notification,
}

impl ModalWindow {
    pub fn new() -> Self {
        Self {
            confirmation_window: ConfirmationWindow::new(),
            create_project_window: CreateProjectWindow::new(),
            file_dialog: FileDialog::new(),
            notification: Notification::new(),
        }
    } 
    pub fn prepare(
        &mut self,
        context: &EGUIContext,
        modal_window_type: &ModalWindowType,
        event_buffer: &mut GUIEventBuffer,
    ) {
        match modal_window_type {
            ModalWindowType::CreateProjectWindow => {
                self.create_project_window.prepare(context, event_buffer); 
            },
            ModalWindowType::FileDialog(item_type) => {
                self.file_dialog.prepare(
                    context, 
                    item_type, 
                    event_buffer
                ); 
            },
            ModalWindowType::Notification(text) => {
                self.notification.prepare(context, event_buffer, text);
            },
            ModalWindowType::ConfirmationWindow { 
                confirmation_text, 
                confirmation_type 
            } => {
                self.confirmation_window.prepare(
                    context, 
                    event_buffer, 
                    confirmation_text, 
                    confirmation_type
                ); 
            },
        }
    }
    pub fn with_create_project_window<F>(&mut self, f: F) 
    where 
        F: FnOnce(&mut CreateProjectWindow)
    {
        f(&mut self.create_project_window);
    }
}
