mod create_new_project_window;
mod file_dialog;
mod notification;
mod waiting_window;

use crate::{
    aliases::{
        EGUIContext,
    },
};
use super::{
    UIEvent,
};
use self::{
    create_new_project_window::CreateNewProjectWindow,
    file_dialog::FileDialogWrap,
    notification::Notification,
    waiting_window::WaitingWindow,
};


#[derive(Debug)]
pub enum ModalWindowState {
    None,
    CreateNewProject,
    FileDialog,
    Notification(String),
    Waiting,
}

impl Default for ModalWindowState {
    fn default() -> Self {
        Self::None
    }
}

pub enum ModalWindowEvent {
    Reset,
    CreateNewProjectReq,
    FileDialogOpenReq,
    FileDialogClosed,
    ShowNotificationReq(String),
    NotificationClosed,
    ShowWaitingWindowReq,
}

#[derive(Default)]
pub struct ModalWindow {
    state: ModalWindowState,
    prev_state: ModalWindowState,
    pub create_new_project_window: CreateNewProjectWindow,
    pub file_dialog: FileDialogWrap,
    pub notification: Notification,
    pub waiting_window: WaitingWindow,
}

impl ModalWindow {
    pub fn prepare(
        &mut self,
        egui_context: &EGUIContext,
    ) -> Vec<UIEvent> {
        let mut ui_events = Vec::with_capacity(4);
                
        let mut events = match &self.state {
            ModalWindowState::None => {
                None
            },
            ModalWindowState::CreateNewProject => {
                let events = self.create_new_project_window.prepare(egui_context);
                Some(events)
            }
            ModalWindowState::FileDialog => {
                let events = self.file_dialog.prepare(egui_context);
                Some(events)
            }
            ModalWindowState::Notification(text) => {
                self.notification.set_notification_text(text);

                let events = self.notification.prepare(egui_context);

                Some(events)
            }
            ModalWindowState::Waiting => {
                self.waiting_window.prepare(egui_context);
                None
            }
        }; 
        
        if let Some(events) = events.take() {
            ui_events.extend(events);
        }

        ui_events
    }

    pub fn on_event(&mut self, event: ModalWindowEvent) {
        match event {
            ModalWindowEvent::Reset => {
                let current_state = std::mem::replace(
                    &mut self.state, 
                    ModalWindowState::None,
                );
                self.prev_state = current_state;
            },
            ModalWindowEvent::CreateNewProjectReq => {
                let current_state = std::mem::replace(
                    &mut self.state, 
                    ModalWindowState::CreateNewProject,
                );
                self.prev_state = current_state; 
            },
            ModalWindowEvent::FileDialogOpenReq => {
                let current_state = std::mem::replace(
                    &mut self.state, 
                    ModalWindowState::FileDialog,
                );
                self.prev_state = current_state;
                self.file_dialog.open_for_pick_directory();
            },
            ModalWindowEvent::ShowNotificationReq(text) => {
                let current_state = std::mem::replace(
                    &mut self.state, 
                    ModalWindowState::Notification(text),
                );
                self.prev_state = current_state;
            },
            ModalWindowEvent::ShowWaitingWindowReq => {
                let current_state = std::mem::replace(
                    &mut self.state, 
                    ModalWindowState::Waiting,
                );
                self.prev_state = current_state;
            },
            ModalWindowEvent::FileDialogClosed => { 
                std::mem::swap(
                    &mut self.state, 
                    &mut self.prev_state
                );
            },
            ModalWindowEvent::NotificationClosed => {
                std::mem::swap(
                    &mut self.state, 
                    &mut self.prev_state
                );
            },
        }        
    }
}

