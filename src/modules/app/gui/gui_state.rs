use super::gui_command::ConfirmationType;

#[derive(Debug, Clone)]
pub enum GUIState {
    Idle,
    ShowModalWindow(ModalWindowType),
}

impl Default for GUIState {
    fn default() -> Self {
        Self::Idle
    } 
}

#[derive(Debug)]
pub enum GUIStateTransform {
    Stay,
    Next(GUIState),
    Prev,
}

#[derive(Debug, Clone)]
pub enum ModalWindowType {
    CreateProjectWindow, 
    OpenProjectWindow,
    FileDialog{
        item_type: ChoosingItemType,
        receiver: FileDialogResponseReceiver,
    },
    Notification(String),
    ConfirmationWindow {
        confirmation_text: String, 
        confirmation_type: ConfirmationType,
    },
    LoadingWindow,
}

#[derive(Debug, Clone)]
pub enum ChoosingItemType {
    File,
    Dir,
}

#[derive(Debug, Clone)]
pub enum FileDialogResponseReceiver {
    CreateProjectWindow,
    OpenProjectWindow,
}
