#[derive(Debug)]
pub enum UIState {
    Idle,
    ModalWindowOpen(ModalWindow),
}

impl Default for UIState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug)]
pub enum ModalWindow {
    CreateNewProjectWindow,
} 
