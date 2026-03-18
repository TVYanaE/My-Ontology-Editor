#[derive(Debug)]
pub enum AppState { 
    Ready,
    Shutdown,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Ready
    }
}
