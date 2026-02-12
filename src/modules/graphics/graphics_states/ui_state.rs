#[derive(Debug, Default)]
pub struct UIState {
    pub ui_general_state: UIGeneralState,
    pub create_new_project_window_state: CreateNewProjectWindowState,
}

#[derive(Debug)]
pub enum UIGeneralState{
    Idle,
    ModalWindowOpen(ModalWindow),
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

#[derive(Debug)]
pub enum CreateNewProjectWindowState {
    MainWindow,
    FileDialog,
}

impl Default for CreateNewProjectWindowState {
    fn default() -> Self {
        Self::MainWindow
    }
}
