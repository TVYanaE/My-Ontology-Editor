use std::path::PathBuf; 

use super::modal_window::ModalWindowType;
use super::gui_command::ConfirmationType;

#[derive(Debug)]
pub enum GUIEvent {
    ExitButtomPressed,
    CreateProjectButtonPressed,
    CreateProjectCanceled,
    FileDialogCanceled,
    NotificationClosed,
    PathSelected(PathBuf),
    OpenModalWindow(ModalWindowType),
    CreateProjectRequest{
        project_name: String,
        project_path: String,
    },
    ConfirmationObtain {
        confirmation_type: ConfirmationType,
        decision: bool,
    }
}

pub struct GUIEventBuffer(Vec<GUIEvent>);

impl GUIEventBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn push(&mut self, event: GUIEvent) {
        self.0.push(event);
    }
    pub fn drain(&mut self) -> impl Iterator<Item = GUIEvent> {
        self.0.drain(..)
    }
}
