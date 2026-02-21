mod confirmation_window;
mod create_new_project_window;
mod file_dialog;
mod notification;
mod waiting_window;

use crate::{
    aliases::{
        EGUIContext,
    },
    modules::{
        graphics_module::{
            ui::{
                ui_state::ModalWindowKind,
                events::UIEvents,
                ui_error::UIError,
            },
        }, 
    },
};

use self::{
    confirmation_window::ConfirmationWindow,
    create_new_project_window::CreateNewProjectWindow,
    file_dialog::FileDialogWrap,
    notification::Notification,
    waiting_window::WaitingWindow,
};



#[derive(Default)]
pub struct ModalWindow {
    pub confirmation_window: ConfirmationWindow,
    pub create_new_project_window: CreateNewProjectWindow,
    pub file_dialog: FileDialogWrap,
    pub notification: Notification,
    pub waiting_window: WaitingWindow, 
}

impl ModalWindow {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext,
        modal_window_kind: ModalWindowKind,
    ) -> Result<UIEvents, UIError> {               
        match modal_window_kind {
            ModalWindowKind::ConfirmationWindow => {
                let events = self.confirmation_window.prepare(egui_context)?;

                Ok(events)
            },
            ModalWindowKind::CreateNewProject => {
                let events = self.create_new_project_window.prepare(egui_context)?;

                Ok(events)         
            },
            ModalWindowKind::Notification => {
                let events = self.notification.prepare(egui_context)?;

                Ok(events)
            },
            ModalWindowKind::FileDialog => {
                let events = self.file_dialog.prepare(egui_context)?;

                Ok(events)
            },
            ModalWindowKind::WaitingWindow => {
                let events = UIEvents::new();
                self.waiting_window.prepare(egui_context);

                Ok(events)
            },
        }   
    } 
}

