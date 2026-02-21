pub enum UIState {
    Default,
    ModalWindow(ModalWindowKind),
    Processing,
}

pub enum Transition {
    Next(UIState),
    Stay,
    Rollback
}

impl Default for UIState {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, Clone)]
pub enum ModalWindowKind {
    CreateNewProject,
    FileDialog,
    Notification,
    WaitingWindow,
    ConfirmationWindow,
}
