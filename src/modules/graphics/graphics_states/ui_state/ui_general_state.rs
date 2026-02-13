#[derive(Debug)]
pub enum UIGeneralState{
    Idle,
    ModalWindowOpen(ModalWindow),
    WaitingBlocingTask,
}

impl Default for UIGeneralState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug)]
pub enum ModalWindow {
    CreateNewProjectWindow,
}
