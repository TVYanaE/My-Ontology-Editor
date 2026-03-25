use crate::modules::app::gui::gui_command::ConfirmationType;

#[derive(Debug, Clone)]
#[derive(Default)]
pub enum GUIState {
    #[default]
    Idle,
    ShowModalWindow(ModalWindowType),
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
    CreateSemanticNodeWindow,
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
