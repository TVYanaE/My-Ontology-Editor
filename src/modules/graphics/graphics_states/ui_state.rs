#[derive(Debug)]
pub enum UIState {
    Idle,
}

impl Default for UIState {
    fn default() -> Self {
        Self::Idle
    }
}
