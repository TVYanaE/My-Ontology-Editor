#[derive(Debug, Clone)]
pub enum CreateNewProjectWindowState {
    Main,
    FileDialog,
    Notification(String),
}

impl Default for CreateNewProjectWindowState {
    fn default() -> Self {
        Self::Main
    }
}
