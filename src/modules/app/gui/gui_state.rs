use super::modal_window::ModalWindowType;

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
