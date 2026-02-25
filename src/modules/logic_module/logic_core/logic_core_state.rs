#[derive(Debug)]
pub enum LogicCoreState {
    Ready,  
    Shutdown,
    Processing,
}

impl Default for LogicCoreState {
    fn default() -> Self {
        Self::Ready
    }
}
